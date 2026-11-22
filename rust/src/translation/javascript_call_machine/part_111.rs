// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyDefinitionsCall;
use super::JavaScriptPropertyDefinitionsState;
use super::JavaScriptPropertyDefinitionsTransition;
use super::ml_array;
use super::ml_string;
use super::read_java_script_property_definition_heap;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_bulk_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionsState,
    transition: self::JavaScriptDescriptorConversionTransition,
) -> self::JavaScriptPropertyDefinitionsTransition {
    match state.clone() {
        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            match transition.clone() {
                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(
                    transition_state,
                ) => {
                    let transition_state = *transition_state;
                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field22071 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field22073 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field22075 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field22077 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field22079 =
                                                shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field22081 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field22083 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22085 =
                                                            shared_constructor_field11;
                                                        {
                                                            let source_ml_source2_nominal_field22087 =
                                                                shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22089 =
                                                                    shared_constructor_field13;
                                                                {
                                                                    let source_ml_source2_nominal_field22091 =
                                                                        shared_constructor_field14
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22093 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22095 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override22099 = None::<self::JavaScriptPropertyDefinitionsCall>;
                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22071.clone(), Box::new(source_ml_source2_nominal_field22073.clone()), Box::new(source_ml_source2_nominal_field22075.clone()), Box::new(source_ml_source2_nominal_field22077.clone()), Box::new(source_ml_source2_nominal_field22079.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]), source_ml_source2_nominal_field22083.clone(), source_ml_source2_nominal_field22085, source_ml_source2_nominal_field22087.clone(), source_ml_source2_nominal_field22089, Some((Some(transition_state.clone()),)), source_ml_source2_nominal_field22093.clone(), Some((source_ml_source2_nominal_override22099.clone(),)))
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
                self::JavaScriptDescriptorConversionTransition::DescriptorCall(
                    transition_state_2,
                    callee,
                    receiver,
                    arguments,
                ) => {
                    let transition_state_2 = *transition_state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    {
                        let call =
                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(
                                Box::new(callee.clone()),
                                Box::new(receiver.clone()),
                                arguments.clone(),
                            );
                        match call.clone() {
                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                let shared_constructor_field1 = *shared_constructor_field1;
                                let shared_constructor_field2 = *shared_constructor_field2;
                                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(Box::new(heap.clone()), Box::new({
                                    let source_ml_source2_nominal_field22101 = shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field22103 = shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field22105 = shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_field22107 = shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field22109 = shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22111 = shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22113 = shared_constructor_field10.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22115 = shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field22117 = shared_constructor_field12.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22119 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_field22121 = shared_constructor_field14.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22123 = shared_constructor_field15.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22125 = shared_constructor_field16.clone();
                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22101.clone(), Box::new(source_ml_source2_nominal_field22103.clone()), Box::new(source_ml_source2_nominal_field22105.clone()), Box::new(source_ml_source2_nominal_field22107.clone()), Box::new(source_ml_source2_nominal_field22109.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]), source_ml_source2_nominal_field22113.clone(), source_ml_source2_nominal_field22115, source_ml_source2_nominal_field22117.clone(), source_ml_source2_nominal_field22119, Some((Some(transition_state_2.clone()),)), source_ml_source2_nominal_field22123.clone(), Some((Some(call.clone()),)))
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
                                }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
                            }
                        }
                    }
                }
                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(descriptor) => {
                    let key = {
                        let source_ml_source2_array_read_value22135 =
                            shared_constructor_field10.clone();
                        {
                            let source_ml_source2_array_read_index22136 =
                                shared_constructor_field11;
                            {
                                let ml_reference_array1112 =
                                    source_ml_source2_array_read_value22135.clone();
                                let ml_reference_position1112 =
                                    source_ml_source2_array_read_index22136;
                                let ml_reference_projection1112 = self::ml_array::optional_at(
                                    &*ml_reference_array1112.borrow(),
                                    self::ml_array::number_index(ml_reference_position1112),
                                );
                                ml_reference_projection1112
                            }
                        }
                    };
                    if match (key.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99,
                                114, 105, 112, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115,
                                32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 32, 107, 101, 121,
                            ]),
                        )
                    } else {
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field22142 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field22144 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field22146 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field22148 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field22150 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field22152 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22154 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22156 =
                                                                shared_constructor_field11;
                                                            {
                                                                let source_ml_source2_nominal_field22158 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field22160 =
                                                                        shared_constructor_field13;
                                                                    {
                                                                        let source_ml_source2_nominal_field22162 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22164 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22166 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override22169 = (shared_constructor_field11 + 1f64);
                                                                                    {
                                                                                        let source_ml_source2_nominal_override22170 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value22139 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_concat_value22140 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone().expect("a proven present value")), descriptor.clone())]));
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), { let ml_reference_value1113 = source_ml_source2_concat_value22139.borrow().clone(); ml_reference_value1113 }), { let ml_reference_value1114 = source_ml_source2_concat_value22140.borrow().clone(); ml_reference_value1114 })
                                                                                            }
                                                                                        }));
                                                                                        {
                                                                                            let source_ml_source2_nominal_override22171 = None::<self::JavaScriptDescriptorConversionState>;
                                                                                            {
                                                                                                let source_ml_source2_nominal_override22172 = None::<self::JavaScriptPropertyDefinitionsCall>;
                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22142.clone(), Box::new(source_ml_source2_nominal_field22144.clone()), Box::new(source_ml_source2_nominal_field22146.clone()), Box::new(source_ml_source2_nominal_field22148.clone()), Box::new(source_ml_source2_nominal_field22150.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100]), source_ml_source2_nominal_field22154.clone(), source_ml_source2_nominal_override22169, source_ml_source2_nominal_override22170.clone(), source_ml_source2_nominal_field22160, Some((source_ml_source2_nominal_override22171.clone(),)), source_ml_source2_nominal_field22164.clone(), Some((source_ml_source2_nominal_override22172.clone(),)))
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
                            }),
                        )
                    }
                }
                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(value) => {
                    let value = *value;
                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsThrow(
                        Box::new(heap.clone()),
                        Box::new(value.clone()),
                    )
                }
                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                    error_class,
                    reason,
                ) => self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(
                    Box::new(heap.clone()),
                    error_class.clone(),
                    reason.clone(),
                ),
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(reason_2) => {
                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                        Box::new(heap.clone()),
                        reason_2.clone(),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_bulk_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionsState,
    transition: self::JavaScriptPropertyDefinitionTransition,
) -> self::JavaScriptPropertyDefinitionsTransition {
    match state.clone() {
        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            {
                let current = self::read_java_script_property_definition_heap(
                    transition.clone(),
                    heap.clone(),
                );
                match transition.clone() {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(
                        transition_state,
                        _,
                    ) => {
                        let transition_state = *transition_state;
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(
                            Box::new(current.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field22184 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field22186 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field22188 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field22190 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field22192 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field22194 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22196 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22198 =
                                                                shared_constructor_field11;
                                                            {
                                                                let source_ml_source2_nominal_field22200 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field22202 =
                                                                        shared_constructor_field13;
                                                                    {
                                                                        let source_ml_source2_nominal_field22204 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22206 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22208 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override22212 = None::<self::JavaScriptPropertyDefinitionsCall>;
                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22184.clone(), Box::new(source_ml_source2_nominal_field22186.clone()), Box::new(source_ml_source2_nominal_field22188.clone()), Box::new(source_ml_source2_nominal_field22190.clone()), Box::new(source_ml_source2_nominal_field22192.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), source_ml_source2_nominal_field22196.clone(), source_ml_source2_nominal_field22198, source_ml_source2_nominal_field22200.clone(), source_ml_source2_nominal_field22202, source_ml_source2_nominal_field22204.clone(), Some((Some(transition_state.clone()),)), Some((source_ml_source2_nominal_override22212.clone(),)))
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
                    self::JavaScriptPropertyDefinitionTransition::DefinitionCall(
                        transition_state_2,
                        callee,
                        receiver,
                        arguments,
                        _,
                    ) => {
                        let transition_state_2 = *transition_state_2;
                        let callee = *callee;
                        let receiver = *receiver;
                        {
                            let call = self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(Box::new(callee.clone()), Box::new(receiver.clone()), arguments.clone());
                            match call.clone() {
                                self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(Box::new(current.clone()), Box::new({
                                        let source_ml_source2_nominal_field22214 = shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field22216 = shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field22218 = shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field22220 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22222 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22224 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22226 = shared_constructor_field10.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field22228 = shared_constructor_field11;
                                                                    {
                                                                        let source_ml_source2_nominal_field22230 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22232 = shared_constructor_field13;
                                                                            {
                                                                                let source_ml_source2_nominal_field22234 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22236 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field22238 = shared_constructor_field16.clone();
                                                                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22214.clone(), Box::new(source_ml_source2_nominal_field22216.clone()), Box::new(source_ml_source2_nominal_field22218.clone()), Box::new(source_ml_source2_nominal_field22220.clone()), Box::new(source_ml_source2_nominal_field22222.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), source_ml_source2_nominal_field22226.clone(), source_ml_source2_nominal_field22228, source_ml_source2_nominal_field22230.clone(), source_ml_source2_nominal_field22232, source_ml_source2_nominal_field22234.clone(), Some((Some(transition_state_2.clone()),)), Some((Some(call.clone()),)))
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
                                    }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
                                }
                            }
                        }
                    }
                    self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(_, _, _) => {
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(
                            Box::new(current.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field22248 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field22250 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field22252 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field22254 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field22256 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field22258 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22260 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22262 =
                                                                shared_constructor_field11;
                                                            {
                                                                let source_ml_source2_nominal_field22264 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field22266 =
                                                                        shared_constructor_field13;
                                                                    {
                                                                        let source_ml_source2_nominal_field22268 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22270 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22272 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override22275 = (shared_constructor_field13 + 1f64);
                                                                                    {
                                                                                        let source_ml_source2_nominal_override22276 = None::<self::JavaScriptPropertyDefinitionState>;
                                                                                        {
                                                                                            let source_ml_source2_nominal_override22277 = None::<self::JavaScriptPropertyDefinitionsCall>;
                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22248.clone(), Box::new(source_ml_source2_nominal_field22250.clone()), Box::new(source_ml_source2_nominal_field22252.clone()), Box::new(source_ml_source2_nominal_field22254.clone()), Box::new(source_ml_source2_nominal_field22256.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), source_ml_source2_nominal_field22260.clone(), source_ml_source2_nominal_field22262, source_ml_source2_nominal_field22264.clone(), source_ml_source2_nominal_override22275, source_ml_source2_nominal_field22268.clone(), Some((source_ml_source2_nominal_override22276.clone(),)), Some((source_ml_source2_nominal_override22277.clone(),)))
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
                            }),
                        )
                    }
                    self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value, _) => {
                        let value = *value;
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsThrow(
                            Box::new(current.clone()),
                            Box::new(value.clone()),
                        )
                    }
                    self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(
                        error_class,
                        reason,
                        _,
                        _,
                    ) => self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(
                        Box::new(current.clone()),
                        error_class.clone(),
                        reason.clone(),
                    ),
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                        reason_2,
                        _,
                    ) => self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                        Box::new(current.clone()),
                        reason_2.clone(),
                    ),
                    self::JavaScriptPropertyDefinitionTransition::DefinitionResourceLimit(
                        _,
                        transition_state_3,
                        reason_3,
                    ) => {
                        let transition_state_3 = *transition_state_3;
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(Box::new(current.clone()), Box::new({
                            let source_ml_source2_nominal_field22279 = shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field22281 = shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field22283 = shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field22285 = shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field22287 = shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field22289 = shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field22291 = shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22293 = shared_constructor_field11;
                                                        {
                                                            let source_ml_source2_nominal_field22295 = shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22297 = shared_constructor_field13;
                                                                {
                                                                    let source_ml_source2_nominal_field22299 = shared_constructor_field14.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22301 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22303 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override22307 = None::<self::JavaScriptPropertyDefinitionsCall>;
                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22279.clone(), Box::new(source_ml_source2_nominal_field22281.clone()), Box::new(source_ml_source2_nominal_field22283.clone()), Box::new(source_ml_source2_nominal_field22285.clone()), Box::new(source_ml_source2_nominal_field22287.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), source_ml_source2_nominal_field22291.clone(), source_ml_source2_nominal_field22293, source_ml_source2_nominal_field22295.clone(), source_ml_source2_nominal_field22297, source_ml_source2_nominal_field22299.clone(), Some((Some(transition_state_3.clone()),)), Some((source_ml_source2_nominal_override22307.clone(),)))
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
                        }), reason_3.clone())
                    }
                }
            }
        }
    }
}
