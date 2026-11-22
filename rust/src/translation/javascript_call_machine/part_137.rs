// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptInstanceOfCall;
use super::JavaScriptInstanceOfState;
use super::JavaScriptInstanceOfTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::accept_java_script_instance_of_method;
use super::accept_java_script_instance_of_prototype;
use super::find_java_script_ordinary_object;
use super::finish_java_script_instance_of;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_instance_of_state_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_instance_of(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptInstanceOfState,
) -> self::JavaScriptInstanceOfTransition {
    match state.clone() {
        self::JavaScriptInstanceOfState::InstanceOfState(
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
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field4,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error =
                        self::read_java_script_instance_of_state_error(heap.clone(), state.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        {
                            let right = (match state.clone() {
                                self::JavaScriptInstanceOfState::InstanceOfState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value26225,
                                    _,
                                ) => ((source_ml_source2_data_property_value26225
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .unwrap_or_else(|| shared_constructor_field7.clone());
                            if !(match (
                                match state.clone() {
                                    self::JavaScriptInstanceOfState::InstanceOfState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value26227,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value26227
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptInstanceOfCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                },
                                None::<()>,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) {
                                self::JavaScriptInstanceOfTransition::InstanceOfCall(
                                    Box::new(heap.clone()),
                                    Box::new(state.clone()),
                                    Box::new(
                                        match (match state.clone() {
                                            self::JavaScriptInstanceOfState::InstanceOfState(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value26228,
                                                _,
                                                _,
                                            ) => ((source_ml_source2_data_property_value26228
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::JavaScriptInstanceOfCall>,)
                                                }))
                                            .0)
                                                .clone(),
                                        })
                                        .clone()
                                        .expect("a proven present value")
                                        {
                                            self::JavaScriptInstanceOfCall::InstanceOfCall(
                                                source_ml_source2_data_property_value26229,
                                                _,
                                                _,
                                            ) => {
                                                let source_ml_source2_data_property_value26229 =
                                                    *source_ml_source2_data_property_value26229;
                                                source_ml_source2_data_property_value26229.clone()
                                            }
                                        },
                                    ),
                                    Box::new(
                                        match (match state.clone() {
                                            self::JavaScriptInstanceOfState::InstanceOfState(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value26230,
                                                _,
                                                _,
                                            ) => ((source_ml_source2_data_property_value26230
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::JavaScriptInstanceOfCall>,)
                                                }))
                                            .0)
                                                .clone(),
                                        })
                                        .clone()
                                        .expect("a proven present value")
                                        {
                                            self::JavaScriptInstanceOfCall::InstanceOfCall(
                                                _,
                                                source_ml_source2_data_property_value26231,
                                                _,
                                            ) => {
                                                let source_ml_source2_data_property_value26231 =
                                                    *source_ml_source2_data_property_value26231;
                                                source_ml_source2_data_property_value26231.clone()
                                            }
                                        },
                                    ),
                                    match (match state.clone() {
                                        self::JavaScriptInstanceOfState::InstanceOfState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value26232,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value26232
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptInstanceOfCall>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value")
                                    {
                                        self::JavaScriptInstanceOfCall::InstanceOfCall(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value26233,
                                        ) => source_ml_source2_data_property_value26233.clone(),
                                    },
                                )
                            } else {
                                if (shared_constructor_field9
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104,
                                        101, 99, 107,
                                    ])))
                                {
                                    if {
                                        let source_ml_source2_s26237 =
                                            shared_constructor_field6.clone();
                                        match source_ml_source2_s26237.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => false,
                                            _ => true,
                                        }
                                    } {
                                        self::finish_java_script_instance_of(
                                            heap.clone(),
                                            state.clone(),
                                            false,
                                        )
                                    } else {
                                        if ({
                                            let source_ml_source2_s26242 =
                                                shared_constructor_field7.clone();
                                            match source_ml_source2_s26242.clone() {
                                                self::JavaScriptSemanticValue::Null => true,
                                                _ => false,
                                            }
                                        } || {
                                            let source_ml_source2_s26245 =
                                                shared_constructor_field7.clone();
                                            match source_ml_source2_s26245.clone() {
                                                self::JavaScriptSemanticValue::Undefined => true,
                                                _ => false,
                                            }
                                        }) {
                                            self::JavaScriptInstanceOfTransition::InstanceOfError(
                                                Box::new(heap.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    67, 97, 110, 110, 111, 116, 32, 99, 111, 110,
                                                    118, 101, 114, 116, 32, 117, 110, 100, 101,
                                                    102, 105, 110, 101, 100, 32, 111, 114, 32, 110,
                                                    117, 108, 108, 32, 116, 111, 32, 111, 98, 106,
                                                    101, 99, 116,
                                                ]),
                                            )
                                        } else {
                                            if {
                                                let source_ml_source2_s26248 =
                                                    shared_constructor_field7.clone();
                                                match source_ml_source2_s26248.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => true,
                                                }
                                            } {
                                                self::finish_java_script_instance_of(
                                                    heap.clone(),
                                                    state.clone(),
                                                    false,
                                                )
                                            } else {
                                                self::JavaScriptInstanceOfTransition::InstanceOfContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field26255 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field26257 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field26259 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field26261 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field26263 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field26265 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field26267 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field26269 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field26271 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field26273 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field26275 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override26278 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override26279 = shared_constructor_field6.clone();
                                                                                                        self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26255.clone(), Box::new(source_ml_source2_nominal_field26257.clone()), Box::new(source_ml_source2_nominal_field26259.clone()), Box::new(source_ml_source2_nominal_field26261.clone()), self::ml_string::Text::from_units(vec![119, 97, 108, 107]), Box::new(source_ml_source2_nominal_override26278.clone()), Box::new(source_ml_source2_nominal_override26279.clone()), source_ml_source2_nominal_field26269.clone(), source_ml_source2_nominal_field26271.clone(), source_ml_source2_nominal_field26273.clone(), source_ml_source2_nominal_field26275.clone())
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
                                } else {
                                    if (shared_constructor_field9
                                        == (self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100,
                                        ])))
                                    {
                                        match right.clone() {
                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                    {
                                                        let get = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), shared_constructor_field8.clone(), right.clone());
                                                        if (((({ let ml_reference_value1405 = get.borrow().clone(); ml_reference_value1405 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                            self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), (({ let ml_reference_value1406 = get.borrow().clone(); ml_reference_value1406 }).0).3)
                                                        } else {
                                                            if (((({ let ml_reference_value1407 = get.borrow().clone(); ml_reference_value1407 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                self::accept_java_script_instance_of_method(heap.clone(), state.clone(), (({ let ml_reference_value1408 = get.borrow().clone(); ml_reference_value1408 }).0).5)
                                                            } else {
                                                                if (((({ let ml_reference_value1409 = get.borrow().clone(); ml_reference_value1409 }).0).0) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                    self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 77, 101, 116, 104, 111, 100, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114]))
                                                                } else {
                                                                    {
                                                                        let call = self::JavaScriptInstanceOfCall::InstanceOfCall(Box::new((({ let ml_reference_value1410 = get.borrow().clone(); ml_reference_value1410 }).0).2), Box::new((({ let ml_reference_value1411 = get.borrow().clone(); ml_reference_value1411 }).0).4), (({ let ml_reference_value1412 = get.borrow().clone(); ml_reference_value1412 }).0).1);
                                                                        match call.clone() {
                                                                            self::JavaScriptInstanceOfCall::InstanceOfCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                self::JavaScriptInstanceOfTransition::InstanceOfCall(Box::new(heap.clone()), Box::new({
                                                                                    let source_ml_source2_nominal_field26285 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field26287 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field26289 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field26291 = shared_constructor_field8.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field26293 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field26295 = shared_constructor_field10.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field26297 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field26299 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field26301 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field26303 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field26305 = shared_constructor_field15.clone();
                                                                                                                            self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26285.clone(), Box::new(source_ml_source2_nominal_field26287.clone()), Box::new(source_ml_source2_nominal_field26289.clone()), Box::new(source_ml_source2_nominal_field26291.clone()), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field26295.clone()), Box::new(source_ml_source2_nominal_field26297.clone()), source_ml_source2_nominal_field26299.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field26303.clone(), source_ml_source2_nominal_field26305.clone())
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
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::JavaScriptInstanceOfTransition::InstanceOfError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![82, 105, 103, 104, 116, 45, 104, 97, 110, 100, 32, 115, 105, 100, 101, 32, 111, 102, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 111, 98, 106, 101, 99, 116]))
                                                }
                                            }
                                    } else {
                                        if (shared_constructor_field9
                                            == (self::ml_string::Text::from_units(vec![
                                                112, 114, 111, 116, 111, 116, 121, 112, 101,
                                            ])))
                                        {
                                            if (match right.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    false
                                                }
                                                _ => true,
                                            } || !(((match right.clone() {
                                                self::JavaScriptSemanticValue::Object(
                                                    _,
                                                    source_ml_source2_data_property_value26316,
                                                ) => Some((
                                                    source_ml_source2_data_property_value26316,
                                                )),
                                                _ => None::<(bool,)>,
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0))
                                            {
                                                self::finish_java_script_instance_of(
                                                    heap.clone(),
                                                    state.clone(),
                                                    false,
                                                )
                                            } else {
                                                {
                                                    let identity_2 = (((match right.clone() {
                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value26317, _) => {
                                                                Some((source_ml_source2_data_property_value26317.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                    if {
                                                        let source_ml_source2_comparison_operand26319 =
                                                            None::<()>;
                                                        match (identity_2.clone(), source_ml_source2_comparison_operand26319.clone()) { (_ml_left, Some(_ml_right)) => false, _ => false }
                                                    } {
                                                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                    } else {
                                                        {
                                                            let bound = {
                                                                let source_ml_source2_array_read_value26322 =
                                                                    shared_constructor_field4
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_array_read_index26323 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), identity_2.clone());
                                                                    {
                                                                        let ml_reference_array1413 = source_ml_source2_array_read_value26322.clone();
                                                                        let ml_reference_position1413 = source_ml_source2_array_read_index26323;
                                                                        let ml_reference_projection1413 = self::ml_array::optional_at(&*ml_reference_array1413.borrow(), self::ml_array::number_index(ml_reference_position1413));
                                                                        ml_reference_projection1413
                                                                    }
                                                                }
                                                            };
                                                            if (!(match (bound.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match bound.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }) {
                                                                    {
                                                                        let empty_path = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
                                                                        {
                                                                            let retained_path = match state.clone() {
                                                                                self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value26332) => {
                                                                                    ((source_ml_source2_data_property_value26332.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,))).0).clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let bound_path = if match (retained_path.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    empty_path.clone()
                                                                                } else {
                                                                                    retained_path.clone().expect("a proven present value")
                                                                                };
                                                                                if (({ let ml_reference_array1414 = bound_path.clone(); let ml_reference_projection1414 = ml_reference_array1414.borrow().len() as f64; ml_reference_projection1414 }) >= (4096f64)) {
                                                                                    self::JavaScriptInstanceOfTransition::InstanceOfResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 100, 101, 108, 101, 103, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 116, 97, 114, 103, 101, 116, 32, 112, 97, 116, 104, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    self::JavaScriptInstanceOfTransition::InstanceOfContinue(Box::new(heap.clone()), Box::new({
                                                                                        let source_ml_source2_nominal_field26337 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field26339 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field26341 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field26343 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field26345 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field26347 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field26349 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field26351 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field26353 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field26355 = shared_constructor_field14.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field26357 = shared_constructor_field15.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override26360 = (((match bound.clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source2_data_property_value26333, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value26333 = *source_ml_source2_data_property_value26333;
                                                                                                                                            Some((source_ml_source2_data_property_value26333.clone(),))
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            None::<(self::JavaScriptSemanticValue,)>
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override26361 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                            let source_ml_source2_concat_value26335 = std::rc::Rc::new(std::cell::RefCell::new(vec![identity_2.clone()]));
                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), { let ml_reference_value1415 = bound_path.borrow().clone(); ml_reference_value1415 }), { let ml_reference_value1416 = source_ml_source2_concat_value26335.borrow().clone(); ml_reference_value1416 })
                                                                                                                                        }));
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override26362 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override26363 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override26364 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
                                                                                                                                                    self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26337.clone(), Box::new(source_ml_source2_nominal_field26339.clone()), Box::new(source_ml_source2_nominal_field26341.clone()), Box::new(source_ml_source2_nominal_field26343.clone()), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]), Box::new(source_ml_source2_nominal_override26362.clone()), Box::new(source_ml_source2_nominal_override26363.clone()), source_ml_source2_nominal_override26364.clone(), source_ml_source2_nominal_field26353.clone(), Some((Some(source_ml_source2_nominal_override26360.clone()),)), Some((Some(source_ml_source2_nominal_override26361.clone()),)))
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
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if {
                                                                        let source_ml_source2_s26366 = shared_constructor_field6.clone();
                                                                        match source_ml_source2_s26366.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    } {
                                                                        self::finish_java_script_instance_of(heap.clone(), state.clone(), false)
                                                                    } else {
                                                                        {
                                                                            let get_2 = self::plan_java_script_ordinary_get(heap.clone(), (((match right.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value26370, _) => {
                                                                                    Some((source_ml_source2_data_property_value26370.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))), right.clone());
                                                                            if (((({ let ml_reference_value1417 = get_2.borrow().clone(); ml_reference_value1417 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), (({ let ml_reference_value1418 = get_2.borrow().clone(); ml_reference_value1418 }).0).3)
                                                                            } else {
                                                                                if (((({ let ml_reference_value1419 = get_2.borrow().clone(); ml_reference_value1419 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                    self::accept_java_script_instance_of_prototype(heap.clone(), state.clone(), (({ let ml_reference_value1420 = get_2.borrow().clone(); ml_reference_value1420 }).0).5)
                                                                                } else {
                                                                                    if (((({ let ml_reference_value1421 = get_2.borrow().clone(); ml_reference_value1421 }).0).0) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 114, 100, 105, 110, 97, 114, 121, 72, 97, 115, 73, 110, 115, 116, 97, 110, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 71, 101, 116]))
                                                                                    } else {
                                                                                        {
                                                                                            let call_2 = self::JavaScriptInstanceOfCall::InstanceOfCall(Box::new((({ let ml_reference_value1422 = get_2.borrow().clone(); ml_reference_value1422 }).0).2), Box::new((({ let ml_reference_value1423 = get_2.borrow().clone(); ml_reference_value1423 }).0).4), (({ let ml_reference_value1424 = get_2.borrow().clone(); ml_reference_value1424 }).0).1);
                                                                                            match call_2.clone() {
                                                                                                self::JavaScriptInstanceOfCall::InstanceOfCall(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2) => {
                                                                                                    let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                                                                                    let shared_constructor_field2_2 = *shared_constructor_field2_2;
                                                                                                    self::JavaScriptInstanceOfTransition::InstanceOfCall(Box::new(heap.clone()), Box::new({
                                                                                                        let source_ml_source2_nominal_field26372 = shared_constructor_field5.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field26374 = shared_constructor_field6.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field26376 = shared_constructor_field7.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field26378 = shared_constructor_field8.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field26380 = shared_constructor_field9.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field26382 = shared_constructor_field10.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field26384 = shared_constructor_field11.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field26386 = shared_constructor_field12.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field26388 = shared_constructor_field13.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field26390 = shared_constructor_field14.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field26392 = shared_constructor_field15.clone();
                                                                                                                                                self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26372.clone(), Box::new(source_ml_source2_nominal_field26374.clone()), Box::new(source_ml_source2_nominal_field26376.clone()), Box::new(source_ml_source2_nominal_field26378.clone()), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field26382.clone()), Box::new(source_ml_source2_nominal_field26384.clone()), source_ml_source2_nominal_field26386.clone(), Some((Some(call_2.clone()),)), source_ml_source2_nominal_field26390.clone(), source_ml_source2_nominal_field26392.clone())
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
                                                                                                    }), Box::new(shared_constructor_field1_2.clone()), Box::new(shared_constructor_field2_2.clone()), shared_constructor_field3_2.clone())
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
                                        } else {
                                            if (shared_constructor_field9
                                                != (self::ml_string::Text::from_units(vec![
                                                    119, 97, 108, 107,
                                                ])))
                                            {
                                                self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let cursor = shared_constructor_field11.clone();
                                                    match cursor.clone() {
                                                            self::JavaScriptSemanticValue::Null => {
                                                                self::finish_java_script_instance_of(heap.clone(), state.clone(), false)
                                                            }
                                                            _ => {
                                                                match cursor.clone() {
                                                                    self::JavaScriptSemanticValue::Object(identity_3, _) => {
                                                                        if self::ml_array::includes(&{ let ml_reference_value1425 = shared_constructor_field12.borrow().clone(); ml_reference_value1425 }, &identity_3.clone(), 0f64, false, |a, b| a == b) {
                                                                            self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 99, 97, 110, 110, 111, 116, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 32, 97, 32, 114, 101, 115, 117, 108, 116, 32, 102, 111, 114, 32, 97, 32, 99, 121, 99, 108, 105, 99, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 97, 116, 104]))
                                                                        } else {
                                                                            if (({ let ml_reference_array1426 = shared_constructor_field12.clone(); let ml_reference_projection1426 = ml_reference_array1426.borrow().len() as f64; ml_reference_projection1426 }) == (4096f64)) {
                                                                                self::JavaScriptInstanceOfTransition::InstanceOfResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 116, 104, 32, 98, 117, 100, 103, 101, 116]))
                                                                            } else {
                                                                                {
                                                                                    let entry = {
                                                                                        let source_ml_source2_array_read_value26409 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index26410 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), identity_3.clone());
                                                                                            { let ml_reference_array1427 = source_ml_source2_array_read_value26409.clone(); let ml_reference_position1427 = source_ml_source2_array_read_index26410; let ml_reference_projection1427 = self::ml_array::optional_at(&*ml_reference_array1427.borrow(), self::ml_array::number_index(ml_reference_position1427)); ml_reference_projection1427 }
                                                                                        }
                                                                                    };
                                                                                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 99, 117, 114, 115, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                                                    } else {
                                                                                        {
                                                                                            let prototype = match entry.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value26411, _, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26411 = *source_ml_source2_data_property_value26411;
                                                                                                    source_ml_source2_data_property_value26411.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value26412, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26412 = *source_ml_source2_data_property_value26412;
                                                                                                    source_ml_source2_data_property_value26412.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value26413, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26413 = *source_ml_source2_data_property_value26413;
                                                                                                    source_ml_source2_data_property_value26413.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value26414, _, _, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26414 = *source_ml_source2_data_property_value26414;
                                                                                                    source_ml_source2_data_property_value26414.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value26415, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26415 = *source_ml_source2_data_property_value26415;
                                                                                                    source_ml_source2_data_property_value26415.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value26416, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26416 = *source_ml_source2_data_property_value26416;
                                                                                                    source_ml_source2_data_property_value26416.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value26417, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26417 = *source_ml_source2_data_property_value26417;
                                                                                                    source_ml_source2_data_property_value26417.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value26418, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26418 = *source_ml_source2_data_property_value26418;
                                                                                                    source_ml_source2_data_property_value26418.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value26419, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26419 = *source_ml_source2_data_property_value26419;
                                                                                                    source_ml_source2_data_property_value26419.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value26420, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26420 = *source_ml_source2_data_property_value26420;
                                                                                                    source_ml_source2_data_property_value26420.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value26421, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26421 = *source_ml_source2_data_property_value26421;
                                                                                                    source_ml_source2_data_property_value26421.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value26422, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26422 = *source_ml_source2_data_property_value26422;
                                                                                                    source_ml_source2_data_property_value26422.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value26423, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value26423 = *source_ml_source2_data_property_value26423;
                                                                                                    source_ml_source2_data_property_value26423.clone()
                                                                                                }
                                                                                            };
                                                                                            match prototype.clone() {
                                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                                    self::finish_java_script_instance_of(heap.clone(), state.clone(), false)
                                                                                                }
                                                                                                _ => {
                                                                                                    if self::test_java_script_semantic_same_value(prototype.clone(), shared_constructor_field10.clone()) {
                                                                                                        self::finish_java_script_instance_of(heap.clone(), state.clone(), true)
                                                                                                    } else {
                                                                                                        self::JavaScriptInstanceOfTransition::InstanceOfContinue(Box::new(heap.clone()), Box::new({
                                                                                                            let source_ml_source2_nominal_field26429 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field26431 = shared_constructor_field6.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field26433 = shared_constructor_field7.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field26435 = shared_constructor_field8.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field26437 = shared_constructor_field9.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field26439 = shared_constructor_field10.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field26441 = shared_constructor_field11.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field26443 = shared_constructor_field12.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field26445 = shared_constructor_field13.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field26447 = shared_constructor_field14.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field26449 = shared_constructor_field15.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override26452 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                            let source_ml_source2_concat_value26426 = shared_constructor_field12.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_concat_value26427 = std::rc::Rc::new(std::cell::RefCell::new(vec![identity_3.clone()]));
                                                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), { let ml_reference_value1428 = source_ml_source2_concat_value26426.borrow().clone(); ml_reference_value1428 }), { let ml_reference_value1429 = source_ml_source2_concat_value26427.borrow().clone(); ml_reference_value1429 })
                                                                                                                                                            }
                                                                                                                                                        }));
                                                                                                                                                        self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26429.clone(), Box::new(source_ml_source2_nominal_field26431.clone()), Box::new(source_ml_source2_nominal_field26433.clone()), Box::new(source_ml_source2_nominal_field26435.clone()), source_ml_source2_nominal_field26437.clone(), Box::new(source_ml_source2_nominal_field26439.clone()), Box::new(prototype.clone()), source_ml_source2_nominal_override26452.clone(), source_ml_source2_nominal_field26445.clone(), source_ml_source2_nominal_field26447.clone(), source_ml_source2_nominal_field26449.clone())
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
                                                                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 99, 117, 114, 115, 111, 114]))
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
