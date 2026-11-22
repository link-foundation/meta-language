// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionForEachCall;
use super::JavaScriptCollectionForEachState;
use super::JavaScriptCollectionForEachTransition;
use super::JavaScriptInstanceOfCall;
use super::JavaScriptInstanceOfState;
use super::JavaScriptInstanceOfTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188;
use super::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189;
use super::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result;
use super::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result;
use super::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_for_each_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1187;
use super::source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1190;
use super::source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1191;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_collection_for_each(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionForEachState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptCollectionForEachTransition {
    {
        let error =
            self::read_java_script_collection_for_each_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if (((match state.clone() {
                self::JavaScriptCollectionForEachState::CollectionForEachState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value25860,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value25860.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                99, 97, 108, 108, 98, 97, 99, 107,
            ]))) || match (
                match state.clone() {
                    self::JavaScriptCollectionForEachState::CollectionForEachState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value25861,
                    ) => ((source_ml_source2_data_property_value25861
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                        101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105,
                        111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 97, 110, 32, 99,
                        111, 109, 112, 108, 101, 116, 101,
                    ]),
                )
            } else {
                if (match completion.clone() {
                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(_) => false,
                    _ => true,
                } && match completion.clone() {
                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(_) => false,
                    _ => true,
                }) {
                    self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108,
                            98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104,
                            101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 114, 109,
                            97, 108, 32, 111, 114, 32, 116, 104, 114, 111, 119, 32, 99, 111, 109,
                            112, 108, 101, 116, 105, 111, 110,
                        ]),
                    )
                } else {
                    {
                        let invalid = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                    source_ml_source2_data_property_value25864,
                                ) => {
                                    let source_ml_source2_data_property_value25864 =
                                        *source_ml_source2_data_property_value25864;
                                    source_ml_source2_data_property_value25864.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    source_ml_source2_data_property_value25865,
                                ) => {
                                    let source_ml_source2_data_property_value25865 =
                                        *source_ml_source2_data_property_value25865;
                                    source_ml_source2_data_property_value25865.clone()
                                }
                            },
                        );
                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    value,
                                ) => {
                                    let value = *value;
                                    self::JavaScriptCollectionForEachTransition::CollectionForEachThrow(Box::new(heap.clone()), Box::new(value.clone()))
                                }
                                _ => {
                                    let call = match state.clone() {
                                            self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25866) => {
                                                ((source_ml_source2_data_property_value25866.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                            }
                                        };
                                    {
                                        let base = match state.clone() {
                                                self::JavaScriptCollectionForEachState::CollectionForEachState(source_ml_source2_nominal_rest_field25867, source_ml_source2_nominal_rest_field25868, source_ml_source2_nominal_rest_field25869, source_ml_source2_nominal_rest_field25870, source_ml_source2_nominal_rest_field25871, source_ml_source2_nominal_rest_field25872, source_ml_source2_nominal_rest_field25873, source_ml_source2_nominal_rest_field25874, _) => {
                                                    let source_ml_source2_nominal_rest_field25868 = *source_ml_source2_nominal_rest_field25868;
                                                    let source_ml_source2_nominal_rest_field25869 = *source_ml_source2_nominal_rest_field25869;
                                                    let source_ml_source2_nominal_rest_field25870 = *source_ml_source2_nominal_rest_field25870;
                                                    let source_ml_source2_nominal_rest_field25873 = *source_ml_source2_nominal_rest_field25873;
                                                    let source_ml_source2_nominal_rest_field25874 = *source_ml_source2_nominal_rest_field25874;
                                                    self::JavaScriptCollectionForEachState::CollectionForEachState(source_ml_source2_nominal_rest_field25867.clone(), Box::new(source_ml_source2_nominal_rest_field25868.clone()), Box::new(source_ml_source2_nominal_rest_field25869.clone()), Box::new(source_ml_source2_nominal_rest_field25870.clone()), source_ml_source2_nominal_rest_field25871, source_ml_source2_nominal_rest_field25872.clone(), Box::new(source_ml_source2_nominal_rest_field25873.clone()), Box::new(source_ml_source2_nominal_rest_field25874.clone()), None::<(Option<self::JavaScriptCollectionForEachCall>,)>)
                                                }
                                            };
                                        match base.clone() {
                                                self::JavaScriptCollectionForEachState::CollectionForEachState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9) => {
                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                    let shared_constructor_field3 = *shared_constructor_field3;
                                                    let shared_constructor_field4 = *shared_constructor_field4;
                                                    let shared_constructor_field7 = *shared_constructor_field7;
                                                    let shared_constructor_field8 = *shared_constructor_field8;
                                                    self::JavaScriptCollectionForEachTransition::CollectionForEachContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field25876 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field25878 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field25880 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field25882 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field25884 = shared_constructor_field5;
                                                                        {
                                                                            let source_ml_source2_nominal_field25886 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field25888 = shared_constructor_field7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field25890 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field25892 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override25895 = self::JavaScriptSemanticValue::Undefined;
                                                                                            {
                                                                                                let source_ml_source2_nominal_override25896 = self::JavaScriptSemanticValue::Undefined;
                                                                                                self::JavaScriptCollectionForEachState::CollectionForEachState(source_ml_source2_nominal_field25876.clone(), Box::new(source_ml_source2_nominal_field25878.clone()), Box::new(source_ml_source2_nominal_field25880.clone()), Box::new(source_ml_source2_nominal_field25882.clone()), source_ml_source2_nominal_field25884, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), Box::new(source_ml_source2_nominal_override25895.clone()), Box::new(source_ml_source2_nominal_override25896.clone()), source_ml_source2_nominal_field25892.clone())
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_instance_of_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptInstanceOfState,
) -> self::ml_string::Text {
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
            _,
            _,
            _,
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
                    let error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        if (!(self::ml_array::includes(
                            &{
                                let ml_reference_value1389 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 115, 116, 97, 110, 99, 101, 111, 102,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 97,
                                            115, 32, 105, 110, 115, 116, 97, 110, 99, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99,
                                            111, 110, 116, 97, 105, 110, 115,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value1389
                            },
                            &shared_constructor_field5.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) || !(self::ml_array::includes(
                            &{
                                let ml_reference_value1390 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100, 32, 118, 97, 108, 117,
                                            101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103,
                                            101, 116, 116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99,
                                            104, 101, 99, 107,
                                        ]),
                                        self::ml_string::Text::from_units(vec![119, 97, 108, 107]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value1390
                            },
                            &shared_constructor_field9.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))) {
                            self::ml_string::Text::from_units(vec![
                                105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116,
                                97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110,
                                100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110,
                                32, 112, 104, 97, 115, 101,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values1184 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        shared_constructor_field6.clone(),
                                        shared_constructor_field7.clone(),
                                        shared_constructor_field8.clone(),
                                        shared_constructor_field10.clone(),
                                        shared_constructor_field11.clone(),
                                    ]));
                                {
                                    let source_ml_source1_temporary_index1184 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s25904 = self::source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1187(heap.clone(), source_ml_source1_temporary_values1184.clone(), source_ml_source1_temporary_index1184.clone());
                                        match source_ml_source2_s25904.clone() {
                                                self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Done => {
                                                    {
                                                        let source_ml_source2_s25921 = if (!(match (match state.clone() {
                                                            self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25922, _) => {
                                                                ((source_ml_source2_data_property_value25922.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match (match state.clone() {
                                                            self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25923) => {
                                                                ((source_ml_source2_data_property_value25923.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                            if ((((((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 111, 110, 116, 97, 105, 110, 115]))) || match (match state.clone() {
                                                                self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25926, _) => {
                                                                    ((source_ml_source2_data_property_value25926.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (match state.clone() {
                                                                self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25927) => {
                                                                    ((source_ml_source2_data_property_value25927.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || (({ let ml_reference_array1391 = (match state.clone() {
                                                                self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25929) => {
                                                                    ((source_ml_source2_data_property_value25929.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value"); let ml_reference_projection1391 = ml_reference_array1391.borrow().len() as f64; ml_reference_projection1391 }) == (0f64))) || (({ let ml_reference_array1392 = (match state.clone() {
                                                                self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25931) => {
                                                                    ((source_ml_source2_data_property_value25931.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value"); let ml_reference_projection1392 = ml_reference_array1392.borrow().len() as f64; ml_reference_projection1392 }) > (4096f64))) || ((self::read_java_script_ordinary_value_error(heap.clone(), (match state.clone() {
                                                                self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25933, _) => {
                                                                    ((source_ml_source2_data_property_value25933.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) {
                                                                self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 108, 101, 103, 97, 116, 101, 100, 32, 116, 97, 114, 103, 101, 116, 32, 112, 97, 116, 104]))
                                                            } else {
                                                                {
                                                                    let target = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source1_temporary_values1185 = (match state.clone() {
                                                                            self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25935) => {
                                                                                ((source_ml_source2_data_property_value25935.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value");
                                                                        {
                                                                            let source_ml_source1_temporary_index1185 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s25936 = self::source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1191(heap.clone(), target.clone(), source_ml_source1_temporary_values1185.clone(), source_ml_source1_temporary_index1185.clone());
                                                                                match source_ml_source2_s25936.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Done(target_2) => {
                                                                                        let target_2 = *target_2;
                                                                                        if !(self::test_java_script_semantic_same_value(target_2.clone(), (match state.clone() {
                                                                                            self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25937, _) => {
                                                                                                ((source_ml_source2_data_property_value25937.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value"))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 100, 101, 108, 101, 103, 97, 116, 101, 100, 32, 116, 97, 114, 103, 101, 116]))
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Next
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Return(source_ml_source1_temporary_result1191) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Return(source_ml_source1_temporary_result1191.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Next
                                                        };
                                                        match source_ml_source2_s25921.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Next => {
                                                                if {
                                                                    let source_ml_source2_s25939 = shared_constructor_field8.clone();
                                                                    match source_ml_source2_s25939.clone() {
                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                } {
                                                                    self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                                                                } else {
                                                                    if ((({ let ml_reference_array1393 = shared_constructor_field12.clone(); let ml_reference_projection1393 = ml_reference_array1393.borrow().len() as f64; ml_reference_projection1393 }) > (4096f64)) || { let ml_values1394 = { let ml_reference_value1395 = shared_constructor_field12.borrow().clone(); ml_reference_value1395 }; let ml_callback1394 = { let heap = heap.clone(); let state = state.clone(); std::rc::Rc::new(move |id: self::ml::Big, index: f64| -> bool { (((self::find_java_script_ordinary_object(match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value25946, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value25946.clone()
                                                                        }
                                                                    }, id.clone())) < (0f64)) || ((self::ml_array::index_of(&{ let ml_reference_value1396 = (match state.clone() {
                                                                        self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, source_ml_source2_data_property_value25950, _, _, _) => {
                                                                            source_ml_source2_data_property_value25950.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value1396 }, &id.clone(), 0f64, false, |a, b| a == b)) != index)) }) }; ml_values1394.iter().cloned().enumerate().any(|(ml_index1394, ml_value1394)| ml_callback1394(ml_value1394, (ml_index1394 as f64))) }) {
                                                                        self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 97, 116, 104])
                                                                    } else {
                                                                        if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![119, 97, 108, 107]))) && ({
                                                                            let source_ml_source2_s25954 = shared_constructor_field10.clone();
                                                                            match source_ml_source2_s25954.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        } || ({
                                                                            let source_ml_source2_s25959 = shared_constructor_field11.clone();
                                                                            match source_ml_source2_s25959.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        } && {
                                                                            let source_ml_source2_s25964 = shared_constructor_field11.clone();
                                                                            match source_ml_source2_s25964.clone() {
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        }))) {
                                                                            self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                                                        } else {
                                                                            {
                                                                                let selected = (((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]))) || (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 118, 97, 108, 117, 101])))) || (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))));
                                                                                if (selected != (!(match (match state.clone() {
                                                                                    self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value25973, _, _) => {
                                                                                        ((source_ml_source2_data_property_value25973.clone().unwrap_or_else(|| (None::<self::JavaScriptInstanceOfCall>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                                                                    self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 111, 110, 108, 121, 32, 119, 104, 105, 108, 101, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100])
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_s26049 = if !(match (match state.clone() {
                                                                                            self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value26050, _, _) => {
                                                                                                ((source_ml_source2_data_property_value26050.clone().unwrap_or_else(|| (None::<self::JavaScriptInstanceOfCall>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            {
                                                                                                let call = (match state.clone() {
                                                                                                    self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value26051, _, _) => {
                                                                                                        ((source_ml_source2_data_property_value26051.clone().unwrap_or_else(|| (None::<self::JavaScriptInstanceOfCall>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value");
                                                                                                match call.clone() {
                                                                                                    self::JavaScriptInstanceOfCall::InstanceOfCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_values1186 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![shared_constructor_field1.clone(), shared_constructor_field2.clone()], { let ml_reference_value1397 = shared_constructor_field3.borrow().clone(); ml_reference_value1397 })));
                                                                                                            {
                                                                                                                let source_ml_source1_temporary_index1186 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let source_ml_source2_s26055 = self::source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1190(heap.clone(), source_ml_source1_temporary_values1186.clone(), source_ml_source1_temporary_index1186.clone());
                                                                                                                    match source_ml_source2_s26055.clone() {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Done => {
                                                                                                                            {
                                                                                                                                let right = (match state.clone() {
                                                                                                                                    self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value26056, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value26056.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().unwrap_or_else(|| shared_constructor_field7.clone());
                                                                                                                                if (({
                                                                                                                                    let source_ml_source2_s26059 = shared_constructor_field1.clone();
                                                                                                                                    match source_ml_source2_s26059.clone() {
                                                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } || !(((match shared_constructor_field1.clone() {
                                                                                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value26064) => {
                                                                                                                                        Some((source_ml_source2_data_property_value26064,))
                                                                                                                                    }
                                                                                                                                    _ => {
                                                                                                                                        None::<(bool,)>
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value")).0)) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), right.clone()))) {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                                                                } else {
                                                                                                                                    if (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 118, 97, 108, 117, 101]))) {
                                                                                                                                        {
                                                                                                                                            let argument = {
                                                                                                                                                let source_ml_source2_array_read_value26069 = shared_constructor_field3.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_array_read_index26070 = 0f64;
                                                                                                                                                    { let ml_reference_array1398 = source_ml_source2_array_read_value26069.clone(); let ml_reference_position1398 = source_ml_source2_array_read_index26070; let ml_reference_projection1398 = self::ml_array::optional_at(&*ml_reference_array1398.borrow(), self::ml_array::number_index(ml_reference_position1398)); ml_reference_projection1398 }
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            if (((({ let ml_reference_array1399 = shared_constructor_field3.clone(); let ml_reference_projection1399 = ml_reference_array1399.borrow().len() as f64; ml_reference_projection1399 }) != (1f64)) || match (argument.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(argument.clone().expect("a proven present value"), shared_constructor_field6.clone()))) {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 102, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Next
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        if (({ let ml_reference_array1400 = shared_constructor_field3.clone(); let ml_reference_projection1400 = ml_reference_array1400.borrow().len() as f64; ml_reference_projection1400 }) != (0f64)) {
                                                                                                                                            match right.clone() {
                                                                                                                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                                                                                                    {
                                                                                                                                                        let owner = {
                                                                                                                                                            let source_ml_source2_array_read_value26079 = shared_constructor_field4.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_array_read_index26080 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), identity.clone());
                                                                                                                                                                { let ml_reference_array1401 = source_ml_source2_array_read_value26079.clone(); let ml_reference_position1401 = source_ml_source2_array_read_index26080; let ml_reference_projection1401 = self::ml_array::optional_at(&*ml_reference_array1401.borrow(), self::ml_array::number_index(ml_reference_position1401)); ml_reference_projection1401 }
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let expected = if (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                                                                                                                shared_constructor_field8.clone()
                                                                                                                                                            } else {
                                                                                                                                                                self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64])))
                                                                                                                                                            };
                                                                                                                                                            {
                                                                                                                                                                let key = {
                                                                                                                                                                    let source_ml_source2_array_read_value26085 = shared_constructor_field3.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_array_read_index26086 = 0f64;
                                                                                                                                                                        { let ml_reference_array1402 = source_ml_source2_array_read_value26085.clone(); let ml_reference_position1402 = source_ml_source2_array_read_index26086; let ml_reference_projection1402 = self::ml_array::optional_at(&*ml_reference_array1402.borrow(), self::ml_array::number_index(ml_reference_position1402)); ml_reference_projection1402 }
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let receiver = {
                                                                                                                                                                        let source_ml_source2_array_read_value26088 = shared_constructor_field3.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_array_read_index26089 = 1f64;
                                                                                                                                                                            { let ml_reference_array1403 = source_ml_source2_array_read_value26088.clone(); let ml_reference_position1403 = source_ml_source2_array_read_index26089; let ml_reference_projection1403 = self::ml_array::optional_at(&*ml_reference_array1403.borrow(), self::ml_array::number_index(ml_reference_position1403)); ml_reference_projection1403 }
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    if (((((((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (((match owner.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                                                                                                            false
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            true
                                                                                                                                                                        }
                                                                                                                                                                    } && match owner.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                                                                                                            false
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            true
                                                                                                                                                                        }
                                                                                                                                                                    }) && match owner.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                                                                                                            false
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            true
                                                                                                                                                                        }
                                                                                                                                                                    }) && match owner.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                                                                                                            false
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            true
                                                                                                                                                                        }
                                                                                                                                                                    })) || !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), (((match owner.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, source_ml_source2_data_property_value26119, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value26119 = *source_ml_source2_data_property_value26119;
                                                                                                                                                                            Some((source_ml_source2_data_property_value26119.clone(),))
                                                                                                                                                                        }
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, source_ml_source2_data_property_value26120, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value26120 = *source_ml_source2_data_property_value26120;
                                                                                                                                                                            Some((source_ml_source2_data_property_value26120.clone(),))
                                                                                                                                                                        }
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, source_ml_source2_data_property_value26121, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value26121 = *source_ml_source2_data_property_value26121;
                                                                                                                                                                            Some((source_ml_source2_data_property_value26121.clone(),))
                                                                                                                                                                        }
                                                                                                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, source_ml_source2_data_property_value26122, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value26122 = *source_ml_source2_data_property_value26122;
                                                                                                                                                                            Some((source_ml_source2_data_property_value26122.clone(),))
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            None::<(self::JavaScriptSemanticValue,)>
                                                                                                                                                                        }
                                                                                                                                                                    }).clone().expect("a proven present value")).0).clone()))) || (({ let ml_reference_array1404 = shared_constructor_field3.clone(); let ml_reference_projection1404 = ml_reference_array1404.borrow().len() as f64; ml_reference_projection1404 }) != (2f64))) || match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), expected.clone()))) || !(self::test_java_script_semantic_same_value(receiver.clone().expect("a proven present value"), right.clone()))) {
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 104, 111, 115, 116, 32, 71, 101, 116, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                                                                                                                                                    } else {
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Next
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                _ => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 104, 111, 115, 116, 32, 71, 101, 116, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Return(source_ml_source1_temporary_result1190) => {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(source_ml_source1_temporary_result1190.clone())
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Next
                                                                                        };
                                                                                        match source_ml_source2_s26049.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Next => {
                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(source_ml_source1_temporary_result1189) => {
                                                                                                source_ml_source1_temporary_result1189.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Return(source_ml_source1_temporary_result1190_2) => {
                                                                source_ml_source1_temporary_result1190_2.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Return(source_ml_source1_temporary_result1191_2) => {
                                                    source_ml_source1_temporary_result1191_2.clone()
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
pub fn begin_java_script_instance_of(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
) -> self::JavaScriptInstanceOfTransition {
    {
        let state = {
            let source_ml_source2_constructor_field26129 = if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    105, 110, 115, 116, 97, 110, 99, 101, 111, 102,
                ]))) {
                self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 111, 110, 116, 97,
                        105, 110, 115,
                    ])))
                {
                    self::ml_string::Text::from_units(vec![
                        112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 101, 99, 107,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                    ])
                }
            };
            {
                let source_ml_source2_constructor_field26130 =
                    self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source2_constructor_field26131 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field26132 =
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
                        self::JavaScriptInstanceOfState::InstanceOfState(
                            operation.clone(),
                            Box::new(left.clone()),
                            Box::new(right.clone()),
                            Box::new(key.clone()),
                            source_ml_source2_constructor_field26129.clone(),
                            Box::new(source_ml_source2_constructor_field26130.clone()),
                            Box::new(source_ml_source2_constructor_field26131.clone()),
                            source_ml_source2_constructor_field26132.clone(),
                            None::<(Option<self::JavaScriptInstanceOfCall>,)>,
                            None::<(Option<self::JavaScriptSemanticValue>,)>,
                            None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>>,)>,
                        )
                    }
                }
            }
        };
        {
            let error = self::read_java_script_instance_of_state_error(heap.clone(), state.clone());
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptInstanceOfTransition::InstanceOfContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_instance_of(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptInstanceOfState,
    value: bool,
) -> self::JavaScriptInstanceOfTransition {
    self::JavaScriptInstanceOfTransition::InstanceOfCompleted(
        Box::new(heap.clone()),
        Box::new(self::JavaScriptSemanticValue::Boolean(value)),
    )
}
