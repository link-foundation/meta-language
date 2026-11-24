// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptArrayObject;
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Result;
use super::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17;
use super::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_array_length;
use super::read_java_script_array_length_key;
use super::source_ml_temporary_plan_java_script_array_length_definition_loop22;
use super::source_ml_temporary_plan_java_script_array_length_definition_loop23;
use super::source_ml_temporary_plan_java_script_array_ordinary_definition_loop16;
use super::test_java_script_array_length;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_ordinary_definition(
    object: self::JavaScriptArrayObject,
    key: self::JavaScriptSemanticValue,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
) -> self::JavaScriptArrayPropertyPlan {
    {
        let at = self::find_java_script_array_property(
            match object.clone() {
                self::JavaScriptArrayObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value399,
                ) => source_ml_source1_data_property_value399.clone(),
            },
            key.clone(),
        );
        {
            let property = {
                let source_ml_source1_array_read_value401 = match object.clone() {
                    self::JavaScriptArrayObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value400,
                    ) => source_ml_source1_data_property_value400.clone(),
                };
                {
                    let source_ml_source1_array_read_index402 = at;
                    {
                        let ml_reference_array138 = source_ml_source1_array_read_value401;
                        let ml_reference_position138 = source_ml_source1_array_read_index402;
                        self::ml_array::optional_at(
                            &*ml_reference_array138.borrow(),
                            self::ml_array::number_index(ml_reference_position138),
                        )
                    }
                }
            };
            {
                let current = if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::create_java_script_property_descriptor(
                        &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
                        false,
                        false,
                    )
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            _,
                            source_ml_source1_data_property_value403,
                        ) => source_ml_source1_data_property_value403.clone(),
                    }
                };
                {
                    let plan = self::plan_java_script_property_definition(
                        !(match (property.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }),
                        match object.clone() {
                            self::JavaScriptArrayObject::ArrayObject(
                                _,
                                _,
                                source_ml_source1_data_property_value404,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value404,
                        },
                        current,
                        proposed.clone(),
                    );
                    if (({ plan.borrow().clone() }).1)
                        == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))
                    {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            ({ plan.borrow().clone() }).2,
                            self::ml_string::Text::from_units(vec![]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    } else if (({ plan.borrow().clone() }).1)
                        == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))
                    {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            true,
                            self::ml_string::Text::from_units(vec![]),
                            self::ml_string::Text::from_units(vec![]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    } else if (at < 0f64)
                        && (({
                            let ml_reference_array142 = match object.clone() {
                                self::JavaScriptArrayObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value406,
                                ) => source_ml_source1_data_property_value406.clone(),
                            };
                            ml_reference_array142.borrow().len() as f64
                        }) >= 4096f64)
                    {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105,
                                101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105,
                                114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100,
                                101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98,
                                117, 100, 103, 101, 116,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                114, 101, 115, 111, 117, 114, 99, 101,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                Box::new(key.clone()),
                                ({ plan.borrow().clone() }).0,
                            );
                            {
                                let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::JavaScriptOrdinaryProperty,
                                >::new(
                                )));
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_temporary_values15 = match object.clone() {
                                            self::JavaScriptArrayObject::ArrayObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value407,
                                            ) => source_ml_source1_data_property_value407.clone(),
                                        };
                                        {
                                            let source_ml_temporary_index15 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s408 = self::source_ml_temporary_plan_java_script_array_ordinary_definition_loop16(at, replacement.clone(), properties.clone(), index, source_ml_temporary_values15, source_ml_temporary_index15);
                                                match source_ml_source1_s408 {
                                                            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Done => {
                                                                {
                                                                    let source_ml_source1_s409 = if at < 0f64 {
                                                                        {
                                                                            let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored18 = { let ml_method_owner144 = properties.clone(); let ml_method_argument144_0 = replacement.clone(); let ml_inserted = vec![ml_method_argument144_0]; let mut ml_method_values144 = ml_method_owner144.borrow_mut(); let ml_length = ml_method_values144.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values144.extend(ml_inserted); ml_length as f64 };
                                                                            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17Next
                                                                        }
                                                                    } else {
                                                                        self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17Next
                                                                    };
                                                                    match source_ml_source1_s409 {
                                                                        self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17Next => {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new({
                                                                                let source_ml_source1_nominal_spread410 = object.clone();
                                                                                {
                                                                                    let source_ml_source1_nominal_field411 = match source_ml_source1_nominal_spread410.clone() {
                                                                                        self::JavaScriptArrayObject::ArrayObject(source_ml_source1_data_property_value412, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value412.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field413 = match source_ml_source1_nominal_spread410.clone() {
                                                                                            self::JavaScriptArrayObject::ArrayObject(_, source_ml_source1_data_property_value414, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value414 = *source_ml_source1_data_property_value414;
                                                                                                source_ml_source1_data_property_value414.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field415 = match source_ml_source1_nominal_spread410.clone() {
                                                                                                self::JavaScriptArrayObject::ArrayObject(_, _, source_ml_source1_data_property_value416, _, _) => {
                                                                                                    source_ml_source1_data_property_value416
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field417 = match source_ml_source1_nominal_spread410.clone() {
                                                                                                    self::JavaScriptArrayObject::ArrayObject(_, _, _, source_ml_source1_data_property_value418, _) => {
                                                                                                        source_ml_source1_data_property_value418
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field419 = match source_ml_source1_nominal_spread410.clone() {
                                                                                                        self::JavaScriptArrayObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value420) => {
                                                                                                            source_ml_source1_data_property_value420.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override421 = properties.clone();
                                                                                                        self::JavaScriptArrayObject::ArrayObject(source_ml_source1_nominal_field411, Box::new(source_ml_source1_nominal_field413), source_ml_source1_nominal_field415, source_ml_source1_nominal_field417, source_ml_source1_nominal_override421)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), ({ plan.borrow().clone() }).3)
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_length_definition(
    object: self::JavaScriptArrayObject,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
    unsigned_length: f64,
    number_length: f64,
) -> self::JavaScriptArrayPropertyPlan {
    if (self::test_java_script_array_length(unsigned_length)) {
        {
            let ml_library_inequality_1 = unsigned_length != number_length;
            if ml_library_inequality_1 {
                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                    Box::new(object.clone()),
                    false,
                    self::ml_string::Text::from_units(vec![
                        73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101,
                        110, 103, 116, 104,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                    ]),
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
                )
            } else {
                {
                    let key = self::read_java_script_array_length_key();
                    {
                        let at = self::find_java_script_array_property(
                            match object.clone() {
                                self::JavaScriptArrayObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value422,
                                ) => source_ml_source1_data_property_value422.clone(),
                            },
                            key.clone(),
                        );
                        {
                            let old = {
                                let source_ml_source1_array_read_value424 = match object.clone() {
                                    self::JavaScriptArrayObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value423,
                                    ) => source_ml_source1_data_property_value423.clone(),
                                };
                                {
                                    let source_ml_source1_array_read_index425 = at;
                                    {
                                        let ml_reference_array146 =
                                            source_ml_source1_array_read_value424;
                                        let ml_reference_position146 =
                                            source_ml_source1_array_read_index425;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array146.borrow(),
                                            self::ml_array::number_index(ml_reference_position146),
                                        )
                                    }
                                }
                            };
                            if match (old.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                    Box::new(object.clone()),
                                    false,
                                    self::ml_string::Text::from_units(vec![
                                        116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114,
                                        114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100,
                                        101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 117,
                                        115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 112, 114,
                                        101, 115, 101, 110, 116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 118, 97, 108, 105, 100,
                                    ]),
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    ))),
                                )
                            } else {
                                {
                                    let length =
                                        self::read_java_script_array_length(object.clone());
                                    {
                                        let descriptor = std::rc::Rc::new(std::cell::RefCell::new(
                                            {
                                                let source_ml_source1_spread426 =
                                                    { proposed.borrow().clone() };
                                                {
                                                    let source_ml_source1_spread_field427 =
                                                        (source_ml_source1_spread426.clone()).0;
                                                    {
                                                        let source_ml_source1_spread_field428 =
                                                            (source_ml_source1_spread426.clone()).1;
                                                        {
                                                            let source_ml_source1_spread_field429 =
                                                                (source_ml_source1_spread426
                                                                    .clone())
                                                                .2;
                                                            {
                                                                let source_ml_source1_spread_field430 =
                                                                    (source_ml_source1_spread426
                                                                        .clone())
                                                                    .3;
                                                                {
                                                                    let source_ml_source1_spread_field431 = (source_ml_source1_spread426.clone()).4;
                                                                    {
                                                                        let source_ml_source1_spread_field432 = (source_ml_source1_spread426.clone()).5;
                                                                        {
                                                                            let source_ml_source1_spread_field433 = (source_ml_source1_spread426.clone()).6;
                                                                            {
                                                                                let source_ml_source1_spread_field434 = (source_ml_source1_spread426.clone()).7;
                                                                                {
                                                                                    let source_ml_source1_spread_field435 = (source_ml_source1_spread426.clone()).8;
                                                                                    {
                                                                                        let source_ml_source1_spread_field436 = (source_ml_source1_spread426.clone()).9;
                                                                                        {
                                                                                            let source_ml_source1_spread_field437 = (source_ml_source1_spread426.clone()).10;
                                                                                            {
                                                                                                let source_ml_source1_spread_field438 = (source_ml_source1_spread426.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source1_field439 = true;
                                                                                                    {
                                                                                                        let source_ml_source1_field440 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                                        (source_ml_source1_spread_field427, source_ml_source1_spread_field428, source_ml_source1_spread_field429, source_ml_source1_spread_field430, source_ml_source1_spread_field431, source_ml_source1_spread_field432, source_ml_source1_spread_field433, source_ml_source1_spread_field434, source_ml_source1_field440, source_ml_source1_field439, source_ml_source1_spread_field437, source_ml_source1_spread_field438)
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
                                        ));
                                        if unsigned_length >= length {
                                        self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), descriptor.clone())
                                    } else if (({ (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value441) => {
                                                source_ml_source1_data_property_value441.clone()
                                            }
                                        }).borrow().clone() }).10) {
                                            {
                                                let new_writable = !(({ descriptor.borrow().clone() }).11) || ({ descriptor.borrow().clone() }).10;
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_spread442 = { descriptor.borrow().clone() };
                                                            {
                                                                let source_ml_source1_spread_field443 = (source_ml_source1_spread442.clone()).0;
                                                                {
                                                                    let source_ml_source1_spread_field444 = (source_ml_source1_spread442.clone()).1;
                                                                    {
                                                                        let source_ml_source1_spread_field445 = (source_ml_source1_spread442.clone()).2;
                                                                        {
                                                                            let source_ml_source1_spread_field446 = (source_ml_source1_spread442.clone()).3;
                                                                            {
                                                                                let source_ml_source1_spread_field447 = (source_ml_source1_spread442.clone()).4;
                                                                                {
                                                                                    let source_ml_source1_spread_field448 = (source_ml_source1_spread442.clone()).5;
                                                                                    {
                                                                                        let source_ml_source1_spread_field449 = (source_ml_source1_spread442.clone()).6;
                                                                                        {
                                                                                            let source_ml_source1_spread_field450 = (source_ml_source1_spread442.clone()).7;
                                                                                            {
                                                                                                let source_ml_source1_spread_field451 = (source_ml_source1_spread442.clone()).8;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field452 = (source_ml_source1_spread442.clone()).9;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field453 = (source_ml_source1_spread442.clone()).10;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field454 = (source_ml_source1_spread442.clone()).11;
                                                                                                            {
                                                                                                                let source_ml_source1_field455 = true;
                                                                                                                (source_ml_source1_spread_field443, source_ml_source1_spread_field444, source_ml_source1_spread_field445, source_ml_source1_spread_field446, source_ml_source1_spread_field447, source_ml_source1_spread_field448, source_ml_source1_spread_field449, source_ml_source1_spread_field450, source_ml_source1_spread_field451, source_ml_source1_spread_field452, source_ml_source1_field455, source_ml_source1_spread_field454)
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
                                                    };
                                                    {
                                                        let initial = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), temporary);
                                                        if (match initial.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value456, _, _, _) => {
                                                                source_ml_source1_data_property_value456
                                                            }
                                                        }) {
                                                            {
                                                                let blocked = -1f64;
                                                                {
                                                                    let source_ml_temporary_values20 = match object.clone() {
                                                                        self::JavaScriptArrayObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value457) => {
                                                                            source_ml_source1_data_property_value457.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index20 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let blocked_2 = self::source_ml_temporary_plan_java_script_array_length_definition_loop22(unsigned_length, blocked, source_ml_temporary_values20, source_ml_temporary_index20);
                                                                            {
                                                                                let final_length = if blocked_2 < 0f64 {
                                                                                    unsigned_length
                                                                                } else {
                                                                                    blocked_2 + 1f64
                                                                                };
                                                                                {
                                                                                    let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                    {
                                                                                        let source_ml_temporary_values21 = match match initial.clone() {
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value458, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value458 = *source_ml_source1_data_property_value458;
                                                                                                source_ml_source1_data_property_value458.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptArrayObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value459) => {
                                                                                                source_ml_source1_data_property_value459.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index21 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s460 = self::source_ml_temporary_plan_java_script_array_length_definition_loop23(new_writable, final_length, properties.clone(), source_ml_temporary_values21, source_ml_temporary_index21);
                                                                                                match source_ml_source1_s460 {
                                                                                                    self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Done => {
                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new({
                                                                                                            let source_ml_source1_nominal_spread462 = match initial.clone() {
                                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value461, _, _, _, _) => {
                                                                                                                    let source_ml_source1_data_property_value461 = *source_ml_source1_data_property_value461;
                                                                                                                    source_ml_source1_data_property_value461.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_nominal_field463 = match source_ml_source1_nominal_spread462.clone() {
                                                                                                                    self::JavaScriptArrayObject::ArrayObject(source_ml_source1_data_property_value464, _, _, _, _) => {
                                                                                                                        source_ml_source1_data_property_value464.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_field465 = match source_ml_source1_nominal_spread462.clone() {
                                                                                                                        self::JavaScriptArrayObject::ArrayObject(_, source_ml_source1_data_property_value466, _, _, _) => {
                                                                                                                            let source_ml_source1_data_property_value466 = *source_ml_source1_data_property_value466;
                                                                                                                            source_ml_source1_data_property_value466.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source1_nominal_field467 = match source_ml_source1_nominal_spread462.clone() {
                                                                                                                            self::JavaScriptArrayObject::ArrayObject(_, _, source_ml_source1_data_property_value468, _, _) => {
                                                                                                                                source_ml_source1_data_property_value468
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source1_nominal_field469 = match source_ml_source1_nominal_spread462.clone() {
                                                                                                                                self::JavaScriptArrayObject::ArrayObject(_, _, _, source_ml_source1_data_property_value470, _) => {
                                                                                                                                    source_ml_source1_data_property_value470
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source1_nominal_field471 = match source_ml_source1_nominal_spread462.clone() {
                                                                                                                                    self::JavaScriptArrayObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value472) => {
                                                                                                                                        source_ml_source1_data_property_value472.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source1_nominal_override473 = properties.clone();
                                                                                                                                    self::JavaScriptArrayObject::ArrayObject(source_ml_source1_nominal_field463, Box::new(source_ml_source1_nominal_field465), source_ml_source1_nominal_field467, source_ml_source1_nominal_field469, source_ml_source1_nominal_override473)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }), (blocked_2 < 0f64), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source1_concat_value477 = match initial.clone() {
                                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value474) => {
                                                                                                                    source_ml_source1_data_property_value474.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value478 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value479 = if !new_writable && !(self::ml_array::includes(&{ (match initial.clone() {
                                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value476) => {
                                                                                                                            source_ml_source1_data_property_value476.clone()
                                                                                                                        }
                                                                                                                    }).borrow().clone() }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b)) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                    };
                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { source_ml_source1_concat_value477.borrow().clone() }), { source_ml_source1_concat_value478.borrow().clone() }), { source_ml_source1_concat_value479.borrow().clone() })
                                                                                                                }
                                                                                                            }
                                                                                                        })))
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
                                                            initial.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
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
        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
            Box::new(object.clone()),
            false,
            self::ml_string::Text::from_units(vec![
                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 116,
                105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110,
                101, 100, 51, 50, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
            ]),
            self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]),
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
        )
    }
}
