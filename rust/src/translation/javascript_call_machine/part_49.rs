// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313;
use super::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318;
use super::MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Result;
use super::accept_java_script_object_assignment_array;
use super::create_java_script_ordinary_set_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_object_spread_property;
use super::read_java_script_object_spread_state_error;
use super::read_java_script_ordinary_own_keys;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_step_java_script_object_spread_loop314;
use super::source_ml_source1_temporary_step_java_script_object_spread_loop319;
use super::step_java_script_array_assignment;
use super::write_java_script_object_assignment_property;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_object_spread(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
) -> self::JavaScriptObjectSpreadTransition {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_object_spread_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        if (shared_constructor_field7
                            == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])))
                        {
                            {
                                let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::JavaScriptSemanticValue,
                                >::new(
                                )));
                                {
                                    let source_ml_source2_s7773 = if {
                                        let source_ml_source2_s7775 =
                                            shared_constructor_field3.clone();
                                        match source_ml_source2_s7775.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => true,
                                            _ => false,
                                        }
                                    } {
                                        {
                                            let own = self::read_java_script_ordinary_own_keys(
                                                heap.clone(),
                                                (((match shared_constructor_field3.clone() {
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source2_data_property_value7780,
                                                        _,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value7780
                                                            .clone(),
                                                    )),
                                                    _ => None::<(self::ml::Big,)>,
                                                })
                                                .clone()
                                                .expect("a proven present value"))
                                                .0)
                                                    .clone(),
                                            );
                                            if (((({
                                                let ml_reference_value625 = own.borrow().clone();
                                                ml_reference_value625
                                            })
                                            .0)
                                                .0)
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Return(Box::new(self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), (({ let ml_reference_value626 = own.borrow().clone(); ml_reference_value626 }).0).0)))
                                            } else {
                                                {
                                                    let keys_2 = (({
                                                        let ml_reference_value627 =
                                                            own.borrow().clone();
                                                        ml_reference_value627
                                                    })
                                                    .0)
                                                        .1;
                                                    self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Next(keys_2.clone())
                                                }
                                            }
                                        }
                                    } else {
                                        if ({
                                            let source_ml_source2_s7782 =
                                                shared_constructor_field3.clone();
                                            match source_ml_source2_s7782.clone() {
                                                self::JavaScriptSemanticValue::Text(_) => true,
                                                _ => false,
                                            }
                                        } || {
                                            let source_ml_source2_s7786 =
                                                shared_constructor_field3.clone();
                                            match source_ml_source2_s7786.clone() {
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    true
                                                }
                                                _ => false,
                                            }
                                        }) {
                                            {
                                                let length =
                                                    self::read_java_script_semantic_text_length(
                                                        shared_constructor_field3.clone(),
                                                    );
                                                if (length > (4096f64)) {
                                                    self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Return(Box::new(self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))))
                                                } else {
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let source_ml_source2_s7791 = self::source_ml_source1_temporary_step_java_script_object_spread_loop314(keys.clone(), length, at);
                                                            match source_ml_source2_s7791.clone() {
                                                                    self::MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Result::MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Done => {
                                                                        self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Next(keys.clone())
                                                                    }
                                                                }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Next(keys.clone())
                                        }
                                    };
                                    match source_ml_source2_s7773.clone() {
                                            self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Next(keys_3) => {
                                                if (({ let ml_reference_array628 = keys_3.clone(); let ml_reference_projection628 = ml_reference_array628.borrow().len() as f64; ml_reference_projection628 }) > (4096f64)) {
                                                    self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 111, 119, 110, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field7793 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field7795 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field7797 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7799 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field7801 = shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field7803 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7805 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field7807 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field7809 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field7811 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field7813 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field7815 = shared_constructor_field13.clone();
                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field7793.clone()), Box::new(source_ml_source2_nominal_field7795.clone()), source_ml_source2_nominal_field7797.clone(), keys_3.clone(), source_ml_source2_nominal_field7801, self::ml_string::Text::from_units(vec![114, 101, 97, 100]), Box::new(source_ml_source2_nominal_field7805.clone()), Box::new(source_ml_source2_nominal_field7807.clone()), source_ml_source2_nominal_field7809.clone(), source_ml_source2_nominal_field7811.clone(), source_ml_source2_nominal_field7813.clone(), source_ml_source2_nominal_field7815.clone())
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
                                            self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313::MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Return(source_ml_source1_temporary_result313) => {
                                                let source_ml_source1_temporary_result313 = *source_ml_source1_temporary_result313;
                                                source_ml_source1_temporary_result313.clone()
                                            }
                                        }
                                }
                            }
                        } else {
                            if !(match (
                                match state.clone() {
                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
                                        source_ml_source2_data_property_value7819,
                                    ) => ((source_ml_source2_data_property_value7819
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptObjectCopyCall>,)
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
                                self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7820) => {
                                            ((source_ml_source2_data_property_value7820.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptObjectCopyCall::ObjectCopyCall(source_ml_source2_data_property_value7821, _, _) => {
                                            let source_ml_source2_data_property_value7821 = *source_ml_source2_data_property_value7821;
                                            source_ml_source2_data_property_value7821.clone()
                                        }
                                    }), Box::new(match (match state.clone() {
                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7822) => {
                                            ((source_ml_source2_data_property_value7822.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptObjectCopyCall::ObjectCopyCall(_, source_ml_source2_data_property_value7823, _) => {
                                            let source_ml_source2_data_property_value7823 = *source_ml_source2_data_property_value7823;
                                            source_ml_source2_data_property_value7823.clone()
                                        }
                                    }), match (match state.clone() {
                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7824) => {
                                            ((source_ml_source2_data_property_value7824.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptObjectCopyCall::ObjectCopyCall(_, _, source_ml_source2_data_property_value7825) => {
                                            source_ml_source2_data_property_value7825.clone()
                                        }
                                    })
                            } else {
                                if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) && !(match (match state.clone() {
                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7828, _) => {
                                            ((source_ml_source2_data_property_value7828.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                        self::accept_java_script_object_assignment_array(heap.clone(), state.clone(), self::step_java_script_array_assignment(heap.clone(), (match state.clone() {
                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7829, _) => {
                                                ((source_ml_source2_data_property_value7829.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")))
                                    } else {
                                        if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) {
                                            self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 103, 101, 116, 116, 101, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 104, 105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        } else {
                                            if (shared_constructor_field6 == ({ let ml_reference_array629 = shared_constructor_field5.clone(); let ml_reference_projection629 = ml_reference_array629.borrow().len() as f64; ml_reference_projection629 })) {
                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field2.clone()))
                                            } else {
                                                {
                                                    let key = {
                                                        let source_ml_source2_array_read_value7839 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_array_read_index7840 = shared_constructor_field6;
                                                            { let ml_reference_array630 = source_ml_source2_array_read_value7839.clone(); let ml_reference_position630 = source_ml_source2_array_read_index7840; let ml_reference_projection630 = self::ml_array::optional_at(&*ml_reference_array630.borrow(), self::ml_array::number_index(ml_reference_position630)); ml_reference_projection630 }
                                                        }
                                                    };
                                                    if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 107, 101, 121]))
                                                    } else {
                                                        if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && {
                                                            let source_ml_source2_comparison_operand7844 = match state.clone() {
                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7843, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value7843.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                }
                                                            };
                                                            match (source_ml_source2_comparison_operand7844.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                        }) {
                                                            self::write_java_script_object_assignment_property(heap.clone(), state.clone(), key.clone().expect("a proven present value"))
                                                        } else {
                                                            if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) {
                                                                if {
                                                                    let source_ml_source2_s7849 = shared_constructor_field2.clone();
                                                                    match source_ml_source2_s7849.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                } {
                                                                    self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 116, 97, 114, 103, 101, 116]))
                                                                } else {
                                                                    {
                                                                        let target = {
                                                                            let source_ml_source2_array_read_value7857 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index7858 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field2.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7856, _) => {
                                                                                        Some((source_ml_source2_data_property_value7856.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                { let ml_reference_array631 = source_ml_source2_array_read_value7857.clone(); let ml_reference_position631 = source_ml_source2_array_read_index7858; let ml_reference_projection631 = self::ml_array::optional_at(&*ml_reference_array631.borrow(), self::ml_array::number_index(ml_reference_position631)); ml_reference_projection631 }
                                                                            }
                                                                        };
                                                                        if match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                            self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 116, 97, 114, 103, 101, 116, 32, 99, 97, 110, 110, 111, 116, 32, 100, 105, 115, 97, 112, 112, 101, 97, 114, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_source2_s7927 = if ((self::find_java_script_ordinary_property(match target.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value7941, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value7941.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value7942, _, _, _) => {
                                                                                        source_ml_source2_data_property_value7942.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value7943) => {
                                                                                        source_ml_source2_data_property_value7943.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value7944, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value7944.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value7945, _) => {
                                                                                        source_ml_source2_data_property_value7945.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value7946, _) => {
                                                                                        source_ml_source2_data_property_value7946.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value7947, _) => {
                                                                                        source_ml_source2_data_property_value7947.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value7948, _) => {
                                                                                        source_ml_source2_data_property_value7948.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7949) => {
                                                                                        source_ml_source2_data_property_value7949.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value7950, _, _) => {
                                                                                        source_ml_source2_data_property_value7950.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value7951, _, _) => {
                                                                                        source_ml_source2_data_property_value7951.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value7952, _, _) => {
                                                                                        source_ml_source2_data_property_value7952.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value7953, _, _) => {
                                                                                        source_ml_source2_data_property_value7953.clone()
                                                                                    }
                                                                                }, key.clone().expect("a proven present value"))) < (0f64)) {
                                                                                    if !(match target.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source2_data_property_value7954, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7954
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source2_data_property_value7955, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7955
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source2_data_property_value7956, _, _) => {
                                                                                            source_ml_source2_data_property_value7956
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value7957, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7957
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source2_data_property_value7958, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7958
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source2_data_property_value7959, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7959
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source2_data_property_value7960, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7960
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source2_data_property_value7961, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7961
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value7962, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7962
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source2_data_property_value7963, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7963
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source2_data_property_value7964, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7964
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source2_data_property_value7965, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7965
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source2_data_property_value7966, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7966
                                                                                        }
                                                                                    }) {
                                                                                        self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Return(Box::new(self::JavaScriptObjectSpreadTransition::ObjectSpreadError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 99, 97, 110, 110, 111, 116, 32, 99, 114, 101, 97, 116, 101, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121]), None::<(Option<self::ml_string::Text>,)>)))
                                                                                    } else {
                                                                                        {
                                                                                            let count = 0f64;
                                                                                            {
                                                                                                let source_ml_source1_temporary_values312 = shared_constructor_field1.clone();
                                                                                                {
                                                                                                    let source_ml_source1_temporary_index312 = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let count_2 = self::source_ml_source1_temporary_step_java_script_object_spread_loop319(count, source_ml_source1_temporary_values312.clone(), source_ml_source1_temporary_index312.clone());
                                                                                                        if ((({ let ml_reference_array632 = match target.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value7983, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value7983.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value7984, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value7984.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value7985) => {
                                                                                                                source_ml_source2_data_property_value7985.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value7986, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value7986.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value7987, _) => {
                                                                                                                source_ml_source2_data_property_value7987.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value7988, _) => {
                                                                                                                source_ml_source2_data_property_value7988.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value7989, _) => {
                                                                                                                source_ml_source2_data_property_value7989.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value7990, _) => {
                                                                                                                source_ml_source2_data_property_value7990.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7991) => {
                                                                                                                source_ml_source2_data_property_value7991.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value7992, _, _) => {
                                                                                                                source_ml_source2_data_property_value7992.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value7993, _, _) => {
                                                                                                                source_ml_source2_data_property_value7993.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value7994, _, _) => {
                                                                                                                source_ml_source2_data_property_value7994.clone()
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value7995, _, _) => {
                                                                                                                source_ml_source2_data_property_value7995.clone()
                                                                                                            }
                                                                                                        }; let ml_reference_projection632 = ml_reference_array632.borrow().len() as f64; ml_reference_projection632 }) >= (4096f64)) || (count_2 >= (16384f64))) {
                                                                                                            self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Return(Box::new(self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))))
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Next
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Next
                                                                                };
                                                                                match source_ml_source2_s7927.clone() {
                                                                                    self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Next => {
                                                                                        {
                                                                                            let defined = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7997, _) => {
                                                                                                    Some((source_ml_source2_data_property_value7997.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(self::ml::Big,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone(), key.clone().expect("a proven present value"), self::create_java_script_ordinary_set_descriptor(shared_constructor_field8.clone(), true));
                                                                                            if (((({ let ml_reference_value633 = defined.borrow().clone(); ml_reference_value633 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new((({ let ml_reference_value634 = defined.borrow().clone(); ml_reference_value634 }).0).1), (({ let ml_reference_value635 = defined.borrow().clone(); ml_reference_value635 }).0).0)
                                                                                            } else {
                                                                                                if !((({ let ml_reference_value636 = defined.borrow().clone(); ml_reference_value636 }).0).3) {
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field7999 = (({ let ml_reference_value637 = defined.borrow().clone(); ml_reference_value637 }).0).1;
                                                                                                        self::JavaScriptObjectSpreadTransition::ObjectSpreadError(Box::new(source_ml_source2_constructor_field7999.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 99, 97, 110, 110, 111, 116, 32, 99, 114, 101, 97, 116, 101, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121]), None::<(Option<self::ml_string::Text>,)>)
                                                                                                    }
                                                                                                } else {
                                                                                                    self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(Box::new((({ let ml_reference_value638 = defined.borrow().clone(); ml_reference_value638 }).0).1), Box::new({
                                                                                                        let source_ml_source2_nominal_field8003 = shared_constructor_field2.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field8005 = shared_constructor_field3.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field8007 = shared_constructor_field4.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field8009 = shared_constructor_field5.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field8011 = shared_constructor_field6;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field8013 = shared_constructor_field7.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field8015 = shared_constructor_field8.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field8017 = shared_constructor_field9.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field8019 = shared_constructor_field10.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field8021 = shared_constructor_field11.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field8023 = shared_constructor_field12.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field8025 = shared_constructor_field13.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override8027 = (shared_constructor_field6 + 1f64);
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override8029 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8003.clone()), Box::new(source_ml_source2_nominal_field8005.clone()), source_ml_source2_nominal_field8007.clone(), source_ml_source2_nominal_field8009.clone(), source_ml_source2_nominal_override8027, self::ml_string::Text::from_units(vec![114, 101, 97, 100]), Box::new(source_ml_source2_nominal_override8029.clone()), Box::new(source_ml_source2_nominal_field8017.clone()), source_ml_source2_nominal_field8019.clone(), source_ml_source2_nominal_field8021.clone(), source_ml_source2_nominal_field8023.clone(), source_ml_source2_nominal_field8025.clone())
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
                                                                                    self::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318::MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Return(source_ml_source1_temporary_result318) => {
                                                                                        let source_ml_source1_temporary_result318 = *source_ml_source1_temporary_result318;
                                                                                        source_ml_source1_temporary_result318.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::read_java_script_object_spread_property(heap.clone(), state.clone(), key.clone().expect("a proven present value"))
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
