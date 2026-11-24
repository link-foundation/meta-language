// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259;
use super::MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Result;
use super::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_array_index;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_unit;
use super::source_ml_source1_temporary_construct_java_script_reflection_text_loop255;
use super::source_ml_source1_temporary_request_java_script_array_transform_allocation_loop254;
use super::test_java_script_array_length_key;
use super::test_java_script_standard_property_reflection_operation;
use super::test_java_script_standard_single_descriptor_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_array_transform_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayTransformState,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptArrayTransformTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            if (length > (4294967295f64)) {
                self::JavaScriptArrayTransformTransition::ArrayTransformError(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101,
                        110, 103, 116, 104,
                    ]),
                )
            } else {
                {
                    let properties = 0f64;
                    {
                        let source_ml_source1_temporary_values253 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source1_temporary_index253 = self::ml::Big::from_i128(0);
                            {
                                let properties_2 = self::source_ml_source1_temporary_request_java_script_array_transform_allocation_loop254(properties, source_ml_source1_temporary_values253.clone(), source_ml_source1_temporary_index253.clone());
                                if ((({
                                    let ml_reference_array474 = shared_constructor_field1.clone();
                                    let ml_reference_projection474 =
                                        ml_reference_array474.borrow().len() as f64;
                                    ml_reference_projection474
                                }) >= (4096f64))
                                    || (properties_2 >= (16384f64)))
                                {
                                    self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    self::JavaScriptArrayTransformTransition::ArrayTransformAllocate(
                                        Box::new(heap.clone()),
                                        Box::new(state.clone()),
                                        Box::new(prototype.clone()),
                                        length,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_property_reflection_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_property_reflection_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_single_descriptor_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_single_descriptor_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_reflection_ordinary_kind(kind: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value475 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101,
                    99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]),
                self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]),
                self::ml_string::Text::from_units(vec![
                    119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116,
                    111, 114, 79, 98, 106, 101, 99, 116,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value475
        },
        kind,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_reflection_text(
    value: &self::ml_string::Text,
) -> self::JavaScriptSemanticValue {
    {
        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let source_ml_source2_s6243 =
                    self::source_ml_source1_temporary_construct_java_script_reflection_text_loop255(
                        value,
                        units.clone(),
                        at,
                    );
                match source_ml_source2_s6243.clone() {
                    self::MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Result::MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Done => {
                        self::JavaScriptSemanticValue::Text(units.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_reflection_property(
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptOrdinaryProperty {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            true,
        );
        {
            let source_ml_source1_temporary_construct_java_script_reflection_property_written257 = {
                let ml_field_owner476 = descriptor.clone();
                let ml_field_value476 = value.clone();
                ml_field_owner476.borrow_mut().0.8 = ml_field_value476;
                ()
            };
            {
                let source_ml_source1_temporary_construct_java_script_reflection_property_written258 = {
                    let ml_field_owner477 = descriptor.clone();
                    let ml_field_value477 = true;
                    ml_field_owner477.borrow_mut().0.10 = ml_field_value477;
                    ()
                };
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                    Box::new(key.clone()),
                    descriptor.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_reflected_descriptor_fields(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            Vec<f64>,
        )>,
    >,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    {
        let fields = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
            self::JavaScriptOrdinaryProperty,
        >::new()));
        {
            let source_ml_source2_s6244 = if ((self::read_java_script_property_descriptor_kind(
                descriptor.clone(),
            )) == (self::ml_string::Text::from_units(vec![
                100, 97, 116, 97,
            ]))) {
                {
                    let source_ml_source1_temporary_construct_java_script_reflected_descriptor_fields_ignored261 = {
                        let ml_method_owner478 = fields.clone();
                        let ml_method_argument478_0 =
                            self::construct_java_script_reflection_property(
                                self::construct_java_script_reflection_text(
                                    &(self::ml_string::Text::from_units(vec![
                                        118, 97, 108, 117, 101,
                                    ])),
                                ),
                                (({
                                    let ml_reference_value479 = descriptor.borrow().clone();
                                    ml_reference_value479
                                })
                                .0)
                                    .8,
                            );
                        let ml_method_argument478_1 =
                            self::construct_java_script_reflection_property(
                                self::construct_java_script_reflection_text(
                                    &(self::ml_string::Text::from_units(vec![
                                        119, 114, 105, 116, 97, 98, 108, 101,
                                    ])),
                                ),
                                self::JavaScriptSemanticValue::Boolean(
                                    (({
                                        let ml_reference_value480 = descriptor.borrow().clone();
                                        ml_reference_value480
                                    })
                                    .0)
                                        .10,
                                ),
                            );
                        let ml_inserted = vec![ml_method_argument478_0, ml_method_argument478_1];
                        let mut ml_method_values478 = ml_method_owner478.borrow_mut();
                        let ml_length = ml_method_values478
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values478.extend(ml_inserted);
                        ml_length as f64
                    };
                    self::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259Next
                }
            } else {
                {
                    let source_ml_source1_temporary_construct_java_script_reflected_descriptor_fields_ignored262 = {
                        let ml_method_owner481 = fields.clone();
                        let ml_method_argument481_0 =
                            self::construct_java_script_reflection_property(
                                self::construct_java_script_reflection_text(
                                    &(self::ml_string::Text::from_units(vec![103, 101, 116])),
                                ),
                                (({
                                    let ml_reference_value482 = descriptor.borrow().clone();
                                    ml_reference_value482
                                })
                                .0)
                                    .4,
                            );
                        let ml_method_argument481_1 =
                            self::construct_java_script_reflection_property(
                                self::construct_java_script_reflection_text(
                                    &(self::ml_string::Text::from_units(vec![115, 101, 116])),
                                ),
                                (({
                                    let ml_reference_value483 = descriptor.borrow().clone();
                                    ml_reference_value483
                                })
                                .0)
                                    .6,
                            );
                        let ml_inserted = vec![ml_method_argument481_0, ml_method_argument481_1];
                        let mut ml_method_values481 = ml_method_owner481.borrow_mut();
                        let ml_length = ml_method_values481
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values481.extend(ml_inserted);
                        ml_length as f64
                    };
                    self::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259Next
                }
            };
            match source_ml_source2_s6244.clone() {
                self::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259::MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259Next => {
                    {
                        let source_ml_source1_temporary_construct_java_script_reflected_descriptor_fields_ignored260 = { let ml_method_owner484 = fields.clone(); let ml_method_argument484_0 = self::construct_java_script_reflection_property(self::construct_java_script_reflection_text(&(self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]))), self::JavaScriptSemanticValue::Boolean((({ let ml_reference_value485 = descriptor.borrow().clone(); ml_reference_value485 }).0).2)); let ml_method_argument484_1 = self::construct_java_script_reflection_property(self::construct_java_script_reflection_text(&(self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]))), self::JavaScriptSemanticValue::Boolean((({ let ml_reference_value486 = descriptor.borrow().clone(); ml_reference_value486 }).0).0)); let ml_inserted = vec![ml_method_argument484_0, ml_method_argument484_1]; let mut ml_method_values484 = ml_method_owner484.borrow_mut(); let ml_length = ml_method_values484.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values484.extend(ml_inserted); ml_length as f64 };
                        fields.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_reflected_own_property(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
) -> Option<self::JavaScriptOrdinaryProperty> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            match input.clone() {
                self::JavaScriptSemanticValue::Object(identity, _) => {
                    let object = {
                        let source_ml_source2_array_read_value6248 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_array_read_index6249 =
                                self::find_java_script_ordinary_object(
                                    shared_constructor_field1.clone(),
                                    identity.clone(),
                                );
                            {
                                let ml_reference_array487 =
                                    source_ml_source2_array_read_value6248.clone();
                                let ml_reference_position487 =
                                    source_ml_source2_array_read_index6249;
                                let ml_reference_projection487 = self::ml_array::optional_at(
                                    &*ml_reference_array487.borrow(),
                                    self::ml_array::number_index(ml_reference_position487),
                                );
                                ml_reference_projection487
                            }
                        }
                    };
                    {
                        let source_ml_source2_optional_chain_receiver6250 = (object.clone(),);
                        match if match (source_ml_source2_optional_chain_receiver6250.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain263 =
                                    ((source_ml_source2_optional_chain_receiver6250.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source2_array_read_value6279 = match source_ml_source1_temporary_optional_chain263.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value6251, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value6251.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value6252, _, _, _) => {
                                                source_ml_source2_data_property_value6252.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value6253) => {
                                                source_ml_source2_data_property_value6253.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value6254, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value6254.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value6255, _) => {
                                                source_ml_source2_data_property_value6255.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value6256, _) => {
                                                source_ml_source2_data_property_value6256.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value6257, _) => {
                                                source_ml_source2_data_property_value6257.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value6258, _) => {
                                                source_ml_source2_data_property_value6258.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value6259) => {
                                                source_ml_source2_data_property_value6259.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value6260, _, _) => {
                                                source_ml_source2_data_property_value6260.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value6261, _, _) => {
                                                source_ml_source2_data_property_value6261.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value6262, _, _) => {
                                                source_ml_source2_data_property_value6262.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value6263, _, _) => {
                                                source_ml_source2_data_property_value6263.clone()
                                            }
                                        };
                                    {
                                        let source_ml_source2_array_read_index6280 =
                                            self::find_java_script_ordinary_property(
                                                {
                                                    let source_ml_source2_nullish_field_receiver6264 =
                                                        (object.clone(),);
                                                    if match (source_ml_source2_nullish_field_receiver6264.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present6265 = ((source_ml_source2_nullish_field_receiver6264.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present6265.clone() {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value6266, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value6266.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value6267, _, _, _) => {
                                                                source_ml_source2_data_property_value6267.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value6268) => {
                                                                source_ml_source2_data_property_value6268.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value6269, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value6269.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value6270, _) => {
                                                                source_ml_source2_data_property_value6270.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value6271, _) => {
                                                                source_ml_source2_data_property_value6271.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value6272, _) => {
                                                                source_ml_source2_data_property_value6272.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value6273, _) => {
                                                                source_ml_source2_data_property_value6273.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value6274) => {
                                                                source_ml_source2_data_property_value6274.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value6275, _, _) => {
                                                                source_ml_source2_data_property_value6275.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value6276, _, _) => {
                                                                source_ml_source2_data_property_value6276.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value6277, _, _) => {
                                                                source_ml_source2_data_property_value6277.clone()
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value6278, _, _) => {
                                                                source_ml_source2_data_property_value6278.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                }
                                                },
                                                key.clone(),
                                            );
                                        {
                                            let ml_reference_array488 =
                                                source_ml_source2_array_read_value6279.clone();
                                            let ml_reference_position488 =
                                                source_ml_source2_array_read_index6280;
                                            let ml_reference_projection488 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array488.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position488,
                                                    ),
                                                );
                                            ml_reference_projection488
                                        }
                                    }
                                }
                            })
                        } else {
                            None::<self::JavaScriptOrdinaryProperty>
                        } {
                            Some(ml_nested) => match ml_nested {
                                Some(ml_present) => Some(ml_present),
                                None => None,
                            },
                            None => None,
                        }
                    }
                }
                _ => {
                    if (match input.clone() {
                        self::JavaScriptSemanticValue::Text(_) => false,
                        _ => true,
                    } && match input.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => false,
                        _ => true,
                    }) {
                        None::<self::JavaScriptOrdinaryProperty>
                    } else {
                        {
                            let length = self::read_java_script_semantic_text_length(input.clone());
                            {
                                let index = self::read_java_script_array_index(key.clone());
                                {
                                    let value = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let enumerable = false;
                                        {
                                            let source_ml_source2_s6284 =
                                                if self::test_java_script_array_length_key(
                                                    key.clone(),
                                                ) {
                                                    {
                                                        let value_2 =
                                                            self::JavaScriptSemanticValue::Number(
                                                                length,
                                                            );
                                                        self::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Next(Box::new(value_2.clone()), enumerable)
                                                    }
                                                } else {
                                                    if ((index >= (0f64)) && (index < length)) {
                                                        {
                                                            let value_3 = self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_java_script_semantic_text_unit(input.clone(), index)])));
                                                            self::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Next(Box::new(value_3.clone()), true)
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Return(None::<self::JavaScriptOrdinaryProperty>)
                                                    }
                                                };
                                            match source_ml_source2_s6284.clone() {
                                                self::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Next(value_4, enumerable_2) => {
                                                    let value_4 = *value_4;
                                                    Some({
                                                        let descriptor = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, enumerable_2);
                                                        {
                                                            let source_ml_source1_temporary_read_java_script_reflected_own_property_written265 = { let ml_field_owner489 = descriptor.clone(); let ml_field_value489 = value_4.clone(); ml_field_owner489.borrow_mut().0.8 = ml_field_value489; () };
                                                            {
                                                                let source_ml_source1_temporary_read_java_script_reflected_own_property_written266 = { let ml_field_owner490 = descriptor.clone(); let ml_field_value490 = false; ml_field_owner490.borrow_mut().0.10 = ml_field_value490; () };
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), descriptor.clone())
                                                            }
                                                        }
                                                    })
                                                }
                                                self::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264::MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Return(source_ml_source1_temporary_result266) => {
                                                    source_ml_source1_temporary_result266.clone()
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
