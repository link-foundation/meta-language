// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246;
use super::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247;
use super::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Result;
use super::create_java_script_ordinary_set_descriptor;
use super::create_java_script_property_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_array_property_definition;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_create_java_script_indexed_data_property_loop245;
use super::source_ml_source1_temporary_create_java_script_indexed_data_property_loop248;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_indexed_data_property(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    index: f64,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptIndexedPropertyCreation {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field4, _, _, _, _, _) => {
            let invalid = self::read_java_script_ordinary_heap_error(heap.clone());
            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(
                    Box::new(heap.clone()),
                    invalid.clone(),
                )
            } else {
                if (((((match target.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } || !(self::ml_math::is_safe_integer(index)))
                    || (index < (0f64)))
                    || {
                        let source_ml_source2_same_value_operand5312 = (-0f64);
                        (((index == source_ml_source2_same_value_operand5312)
                            && ((index != (0f64))
                                || ((1f64 / index)
                                    == (1f64 / source_ml_source2_same_value_operand5312))))
                            || ((index != index)
                                && (source_ml_source2_same_value_operand5312
                                    != source_ml_source2_same_value_operand5312)))
                    })
                    || ((self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        target.clone(),
                    )) != (self::ml_string::Text::from_units(vec![]))))
                    || ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                        != (self::ml_string::Text::from_units(vec![]))))
                {
                    self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            105, 110, 100, 101, 120, 101, 100, 32, 112, 114, 111, 112, 101, 114,
                            116, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                            97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                            32, 79, 98, 106, 101, 99, 116, 44, 32, 105, 110, 100, 101, 120, 32, 97,
                            110, 100, 32, 118, 97, 108, 117, 101,
                        ]),
                    )
                } else {
                    {
                        let object = {
                            let source_ml_source2_array_read_value5316 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_array_read_index5317 =
                                    self::find_java_script_ordinary_object(
                                        shared_constructor_field4.clone(),
                                        (((match target.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value5315,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value5315
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                    );
                                {
                                    let ml_reference_array423 =
                                        source_ml_source2_array_read_value5316.clone();
                                    let ml_reference_position423 =
                                        source_ml_source2_array_read_index5317;
                                    let ml_reference_projection423 = self::ml_array::optional_at(
                                        &*ml_reference_array423.borrow(),
                                        self::ml_array::number_index(ml_reference_position423),
                                    );
                                    ml_reference_projection423
                                }
                            }
                        };
                        if (match (object.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || !(self::ml_array::includes(
                            &{
                                let ml_reference_value424 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106,
                                            101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99,
                                            116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105,
                                            111, 110, 79, 98, 106, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            109, 97, 112, 79, 98, 106, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            115, 101, 116, 79, 98, 106, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99,
                                            116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99,
                                            116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73,
                                            116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99,
                                            116,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value424
                            },
                            &match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
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
                                ) => self::ml_string::Text::from_units(vec![
                                    99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                                ]),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110,
                                    79, 98, 106, 101, 99, 116,
                                ]),
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                                    ])
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                                ) => self::ml_string::Text::from_units(vec![
                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101,
                                    114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116,
                                ]),
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        109, 97, 112, 79, 98, 106, 101, 99, 116,
                                    ])
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        115, 101, 116, 79, 98, 106, 101, 99, 116,
                                    ])
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                                    ])
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                                    ])
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99,
                                    116,
                                ]),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101,
                                    99, 116,
                                ]),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116,
                                ]),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101,
                                    99, 116,
                                ]),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101,
                                    114, 79, 98, 106, 101, 99, 116,
                                ]),
                            },
                            0f64,
                            false,
                            |a, b| a == b,
                        ))) {
                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    105, 110, 100, 101, 120, 101, 100, 32, 112, 114, 111, 112, 101,
                                    114, 116, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32,
                                    114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 111, 112, 97, 113, 117, 101, 32,
                                    104, 111, 115, 116, 47, 80, 114, 111, 120, 121, 32, 100, 101,
                                    102, 105, 110, 105, 116, 105, 111, 110, 32, 112, 114, 111, 116,
                                    111, 99, 111, 108,
                                ]),
                            )
                        } else {
                            {
                                let name = self::ml_string::Text::from_text(
                                    &self::ml_number::js_number(index),
                                );
                                {
                                    let units = std::rc::Rc::new(std::cell::RefCell::new(
                                        Vec::<f64>::new(),
                                    ));
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s5319 = self::source_ml_source1_temporary_create_java_script_indexed_data_property_loop245(&(name), units.clone(), at);
                                            match source_ml_source2_s5319.clone() {
                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Result::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Done => {
                                                        {
                                                            let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                            {
                                                                let descriptor = self::create_java_script_ordinary_set_descriptor(value.clone(), true);
                                                                {
                                                                    let at_2 = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value5320, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value5320.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value5321, _, _, _) => {
                                                                            source_ml_source2_data_property_value5321.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value5322) => {
                                                                            source_ml_source2_data_property_value5322.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value5323, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value5323.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value5324, _) => {
                                                                            source_ml_source2_data_property_value5324.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value5325, _) => {
                                                                            source_ml_source2_data_property_value5325.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value5326, _) => {
                                                                            source_ml_source2_data_property_value5326.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value5327, _) => {
                                                                            source_ml_source2_data_property_value5327.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value5328) => {
                                                                            source_ml_source2_data_property_value5328.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value5329, _, _) => {
                                                                            source_ml_source2_data_property_value5329.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value5330, _, _) => {
                                                                            source_ml_source2_data_property_value5330.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value5331, _, _) => {
                                                                            source_ml_source2_data_property_value5331.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value5332, _, _) => {
                                                                            source_ml_source2_data_property_value5332.clone()
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                                                                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(Box::new(heap.clone()), key_error.clone())
                                                                        } else {
                                                                            {
                                                                                let source_ml_source2_s5459 = match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, extensible, _, properties, _, _, _, _, _) => {
                                                                                        {
                                                                                            let current = ({
                                                                                                let source_ml_source2_optional_chain_receiver5533 = ({ let ml_reference_array425 = properties.clone(); let ml_reference_position425 = at_2; let ml_reference_projection425 = self::ml_array::optional_at(&*ml_reference_array425.borrow(), self::ml_array::number_index(ml_reference_position425)); ml_reference_projection425 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5533.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243 = ((source_ml_source2_optional_chain_receiver5533.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5534) => {
                                                                                                                source_ml_source2_data_property_value5534.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible, current.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value426 = plan.borrow().clone(); ml_reference_value426 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, extensible_2, _, properties_2, _, _, _) => {
                                                                                        {
                                                                                            let current_2 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5537 = ({ let ml_reference_array427 = properties_2.clone(); let ml_reference_position427 = at_2; let ml_reference_projection427 = self::ml_array::optional_at(&*ml_reference_array427.borrow(), self::ml_array::number_index(ml_reference_position427)); ml_reference_projection427 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5537.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_2 = ((source_ml_source2_optional_chain_receiver5537.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_2.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5538) => {
                                                                                                                source_ml_source2_data_property_value5538.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_2 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_2, current_2.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value428 = plan_2.borrow().clone(); ml_reference_value428 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                        {
                                                                                            let plan_3 = self::plan_java_script_array_property_definition(object.clone().expect("a proven present value"), key.clone(), descriptor.clone());
                                                                                            match plan_3.clone() {
                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
                                                                                                    if (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]))) {
                                                                                                        self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(Box::new(heap.clone()), shared_constructor_field2.clone())))
                                                                                                    } else {
                                                                                                        if (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(Box::new(heap.clone()), shared_constructor_field2.clone())))
                                                                                                        } else {
                                                                                                            if !shared_constructor_field1 {
                                                                                                                self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, extensible_3, _, properties_3, _, _, _, _, _, _) => {
                                                                                        {
                                                                                            let current_3 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5548 = ({ let ml_reference_array429 = properties_3.clone(); let ml_reference_position429 = at_2; let ml_reference_projection429 = self::ml_array::optional_at(&*ml_reference_array429.borrow(), self::ml_array::number_index(ml_reference_position429)); ml_reference_projection429 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5548.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_3 = ((source_ml_source2_optional_chain_receiver5548.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_3.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5549) => {
                                                                                                                source_ml_source2_data_property_value5549.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_4 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_3, current_3.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value430 = plan_4.borrow().clone(); ml_reference_value430 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, extensible_4, _, properties_4, _) => {
                                                                                        {
                                                                                            let current_4 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5552 = ({ let ml_reference_array431 = properties_4.clone(); let ml_reference_position431 = at_2; let ml_reference_projection431 = self::ml_array::optional_at(&*ml_reference_array431.borrow(), self::ml_array::number_index(ml_reference_position431)); ml_reference_projection431 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5552.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_4 = ((source_ml_source2_optional_chain_receiver5552.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_4.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5553) => {
                                                                                                                source_ml_source2_data_property_value5553.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_5 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_4, current_4.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value432 = plan_5.borrow().clone(); ml_reference_value432 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, extensible_5, _, properties_5, _) => {
                                                                                        {
                                                                                            let current_5 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5556 = ({ let ml_reference_array433 = properties_5.clone(); let ml_reference_position433 = at_2; let ml_reference_projection433 = self::ml_array::optional_at(&*ml_reference_array433.borrow(), self::ml_array::number_index(ml_reference_position433)); ml_reference_projection433 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5556.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_5 = ((source_ml_source2_optional_chain_receiver5556.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_5.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5557) => {
                                                                                                                source_ml_source2_data_property_value5557.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_6 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_5, current_5.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value434 = plan_6.borrow().clone(); ml_reference_value434 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, extensible_6, _, properties_6, _) => {
                                                                                        {
                                                                                            let current_6 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5560 = ({ let ml_reference_array435 = properties_6.clone(); let ml_reference_position435 = at_2; let ml_reference_projection435 = self::ml_array::optional_at(&*ml_reference_array435.borrow(), self::ml_array::number_index(ml_reference_position435)); ml_reference_projection435 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5560.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_6 = ((source_ml_source2_optional_chain_receiver5560.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_6.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5561) => {
                                                                                                                source_ml_source2_data_property_value5561.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_7 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_6, current_6.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value436 = plan_7.borrow().clone(); ml_reference_value436 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, extensible_7, _, properties_7, _) => {
                                                                                        {
                                                                                            let current_7 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5564 = ({ let ml_reference_array437 = properties_7.clone(); let ml_reference_position437 = at_2; let ml_reference_projection437 = self::ml_array::optional_at(&*ml_reference_array437.borrow(), self::ml_array::number_index(ml_reference_position437)); ml_reference_projection437 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5564.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_7 = ((source_ml_source2_optional_chain_receiver5564.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_7.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5565) => {
                                                                                                                source_ml_source2_data_property_value5565.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_8 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_7, current_7.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value438 = plan_8.borrow().clone(); ml_reference_value438 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, extensible_8, _, _, _, _, _, properties_8) => {
                                                                                        {
                                                                                            let current_8 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5568 = ({ let ml_reference_array439 = properties_8.clone(); let ml_reference_position439 = at_2; let ml_reference_projection439 = self::ml_array::optional_at(&*ml_reference_array439.borrow(), self::ml_array::number_index(ml_reference_position439)); ml_reference_projection439 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5568.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_8 = ((source_ml_source2_optional_chain_receiver5568.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_8.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5569) => {
                                                                                                                source_ml_source2_data_property_value5569.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_9 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_8, current_8.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value440 = plan_9.borrow().clone(); ml_reference_value440 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, extensible_9, _, properties_9, _, _) => {
                                                                                        {
                                                                                            let current_9 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5572 = ({ let ml_reference_array441 = properties_9.clone(); let ml_reference_position441 = at_2; let ml_reference_projection441 = self::ml_array::optional_at(&*ml_reference_array441.borrow(), self::ml_array::number_index(ml_reference_position441)); ml_reference_projection441 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5572.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_9 = ((source_ml_source2_optional_chain_receiver5572.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_9.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5573) => {
                                                                                                                source_ml_source2_data_property_value5573.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_10 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_9, current_9.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value442 = plan_10.borrow().clone(); ml_reference_value442 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, extensible_10, _, properties_10, _, _) => {
                                                                                        {
                                                                                            let current_10 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5576 = ({ let ml_reference_array443 = properties_10.clone(); let ml_reference_position443 = at_2; let ml_reference_projection443 = self::ml_array::optional_at(&*ml_reference_array443.borrow(), self::ml_array::number_index(ml_reference_position443)); ml_reference_projection443 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5576.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_10 = ((source_ml_source2_optional_chain_receiver5576.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_10.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5577) => {
                                                                                                                source_ml_source2_data_property_value5577.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_11 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_10, current_10.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value444 = plan_11.borrow().clone(); ml_reference_value444 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, extensible_11, _, properties_11, _, _) => {
                                                                                        {
                                                                                            let current_11 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5580 = ({ let ml_reference_array445 = properties_11.clone(); let ml_reference_position445 = at_2; let ml_reference_projection445 = self::ml_array::optional_at(&*ml_reference_array445.borrow(), self::ml_array::number_index(ml_reference_position445)); ml_reference_projection445 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5580.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_11 = ((source_ml_source2_optional_chain_receiver5580.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_11.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5581) => {
                                                                                                                source_ml_source2_data_property_value5581.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_12 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_11, current_11.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value446 = plan_12.borrow().clone(); ml_reference_value446 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, extensible_12, _, properties_12, _, _) => {
                                                                                        {
                                                                                            let current_12 = ({
                                                                                                let source_ml_source2_optional_chain_receiver5584 = ({ let ml_reference_array447 = properties_12.clone(); let ml_reference_position447 = at_2; let ml_reference_projection447 = self::ml_array::optional_at(&*ml_reference_array447.borrow(), self::ml_array::number_index(ml_reference_position447)); ml_reference_projection447 },);
                                                                                                if match (source_ml_source2_optional_chain_receiver5584.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    Some({
                                                                                                        let source_ml_source1_temporary_optional_chain243_12 = ((source_ml_source2_optional_chain_receiver5584.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source1_temporary_optional_chain243_12.clone() {
                                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value5585) => {
                                                                                                                source_ml_source2_data_property_value5585.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    })
                                                                                                } else {
                                                                                                    None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false));
                                                                                            {
                                                                                                let plan_13 = self::plan_java_script_property_definition((at_2 >= (0f64)), extensible_12, current_12.clone(), descriptor.clone());
                                                                                                if (((({ let ml_reference_value448 = plan_13.borrow().clone(); ml_reference_value448 }).0).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100]))))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s5459.clone() {
                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next => {
                                                                                        {
                                                                                            let source_ml_source2_s5613 = if (at_2 < (0f64)) {
                                                                                                {
                                                                                                    let total = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values244 = shared_constructor_field4.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index244 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let total_2 = self::source_ml_source1_temporary_create_java_script_indexed_data_property_loop248(total, source_ml_source1_temporary_values244.clone(), source_ml_source1_temporary_index244.clone());
                                                                                                                if ((({ let ml_reference_array449 = match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value5628, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5628.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value5629, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5629.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value5630) => {
                                                                                                                        source_ml_source2_data_property_value5630.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value5631, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5631.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value5632, _) => {
                                                                                                                        source_ml_source2_data_property_value5632.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value5633, _) => {
                                                                                                                        source_ml_source2_data_property_value5633.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value5634, _) => {
                                                                                                                        source_ml_source2_data_property_value5634.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value5635, _) => {
                                                                                                                        source_ml_source2_data_property_value5635.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value5636) => {
                                                                                                                        source_ml_source2_data_property_value5636.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value5637, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5637.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value5638, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5638.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value5639, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5639.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value5640, _, _) => {
                                                                                                                        source_ml_source2_data_property_value5640.clone()
                                                                                                                    }
                                                                                                                }; let ml_reference_projection449 = ml_reference_array449.borrow().len() as f64; ml_reference_projection449 }) >= (4096f64)) || (total_2 >= (16384f64))) {
                                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Return(Box::new(self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))))
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Next
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Next
                                                                                            };
                                                                                            match source_ml_source2_s5613.clone() {
                                                                                                self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Next => {
                                                                                                    {
                                                                                                        let defined = self::define_java_script_ordinary_property(heap.clone(), (((match target.clone() {
                                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value5641, _) => {
                                                                                                                Some((source_ml_source2_data_property_value5641.clone(),))
                                                                                                            }
                                                                                                            _ => {
                                                                                                                None::<(self::ml::Big,)>
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value")).0).clone(), key.clone(), descriptor.clone());
                                                                                                        if (({ let ml_reference_value450 = defined.borrow().clone(); ml_reference_value450 }).0).3 {
                                                                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(Box::new((({ let ml_reference_value451 = defined.borrow().clone(); ml_reference_value451 }).0).1))
                                                                                                        } else {
                                                                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(Box::new((({ let ml_reference_value452 = defined.borrow().clone(); ml_reference_value452 }).0).1), {
                                                                                                                let source_ml_source2_logical5642 = (({ let ml_reference_value453 = defined.borrow().clone(); ml_reference_value453 }).0).0;
                                                                                                                if !source_ml_source2_logical5642.0.is_empty() {
                                                                                                                    source_ml_source2_logical5642.clone()
                                                                                                                } else {
                                                                                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 100, 101, 120, 101, 100, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 104, 97, 110, 103, 101, 100, 32, 97, 102, 116, 101, 114, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 108, 97, 110])
                                                                                                                }
                                                                                                            })
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Return(source_ml_source1_temporary_result247) => {
                                                                                                    let source_ml_source1_temporary_result247 = *source_ml_source1_temporary_result247;
                                                                                                    source_ml_source1_temporary_result247.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(source_ml_source1_temporary_result248) => {
                                                                                        let source_ml_source1_temporary_result248 = *source_ml_source1_temporary_result248;
                                                                                        source_ml_source1_temporary_result248.clone()
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
