// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21;
use super::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::source_ml_source1_temporary_plan_java_script_array_ordinary_definition_loop20;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_ordinary_definition(
    object: self::JavaScriptOrdinaryObject,
    key: self::JavaScriptSemanticValue,
    proposed: std::rc::Rc<
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
) -> self::JavaScriptArrayPropertyPlan {
    {
        let at = self::find_java_script_array_property(
            match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value225,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value225.clone(),
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value226,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value226.clone(),
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value227,
                ) => source_ml_source2_data_property_value227.clone(),
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value228,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value228.clone(),
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value229,
                    _,
                ) => source_ml_source2_data_property_value229.clone(),
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value230,
                    _,
                ) => source_ml_source2_data_property_value230.clone(),
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value231,
                    _,
                ) => source_ml_source2_data_property_value231.clone(),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value232,
                    _,
                ) => source_ml_source2_data_property_value232.clone(),
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value233,
                ) => source_ml_source2_data_property_value233.clone(),
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value234,
                    _,
                    _,
                ) => source_ml_source2_data_property_value234.clone(),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value235,
                    _,
                    _,
                ) => source_ml_source2_data_property_value235.clone(),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value236,
                    _,
                    _,
                ) => source_ml_source2_data_property_value236.clone(),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value237,
                    _,
                    _,
                ) => source_ml_source2_data_property_value237.clone(),
            },
            key.clone(),
        );
        {
            let property = {
                let source_ml_source2_array_read_value251 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value238,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value238.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value239,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value239.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value240,
                    ) => source_ml_source2_data_property_value240.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value241,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value241.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value242,
                        _,
                    ) => source_ml_source2_data_property_value242.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value243,
                        _,
                    ) => source_ml_source2_data_property_value243.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value244,
                        _,
                    ) => source_ml_source2_data_property_value244.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value245,
                        _,
                    ) => source_ml_source2_data_property_value245.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value246,
                    ) => source_ml_source2_data_property_value246.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value247,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value247.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value248,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value248.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value249,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value249.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value250,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value250.clone(),
                };
                {
                    let ml_reference_array145 = source_ml_source2_array_read_value251.clone();
                    let ml_reference_position145 = at;
                    let ml_reference_projection145 = self::ml_array::optional_at(
                        &*ml_reference_array145.borrow(),
                        self::ml_array::number_index(ml_reference_position145),
                    );
                    ml_reference_projection145
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
                            source_ml_source2_data_property_value253,
                        ) => source_ml_source2_data_property_value253.clone(),
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
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                source_ml_source2_data_property_value254,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value254,
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                source_ml_source2_data_property_value255,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value255,
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                source_ml_source2_data_property_value256,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value256,
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                source_ml_source2_data_property_value257,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value257,
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                source_ml_source2_data_property_value258,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value258,
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                source_ml_source2_data_property_value259,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value259,
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                source_ml_source2_data_property_value260,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value260,
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                source_ml_source2_data_property_value261,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value261,
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                source_ml_source2_data_property_value262,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value262,
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                source_ml_source2_data_property_value263,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value263,
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                source_ml_source2_data_property_value264,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value264,
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                source_ml_source2_data_property_value265,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value265,
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                source_ml_source2_data_property_value266,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value266,
                        },
                        current.clone(),
                        proposed.clone(),
                    );
                    if (((({
                        let ml_reference_value146 = plan.borrow().clone();
                        ml_reference_value146
                    })
                    .0)
                        .1)
                        == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])))
                    {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            (({
                                let ml_reference_value147 = plan.borrow().clone();
                                ml_reference_value147
                            })
                            .0)
                                .2,
                            self::ml_string::Text::from_units(vec![]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    } else {
                        if (((({
                            let ml_reference_value148 = plan.borrow().clone();
                            ml_reference_value148
                        })
                        .0)
                            .1)
                            == (self::ml_string::Text::from_units(vec![107, 101, 101, 112])))
                        {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                Box::new(object.clone()),
                                true,
                                self::ml_string::Text::from_units(vec![]),
                                self::ml_string::Text::from_units(vec![]),
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                ))),
                            )
                        } else {
                            if ((at < (0f64))
                                && (({
                                    let ml_reference_array149 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value280, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value280.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value281, _, _, _) => {
                                    source_ml_source2_data_property_value281.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value282) => {
                                    source_ml_source2_data_property_value282.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value283, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value283.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value284, _) => {
                                    source_ml_source2_data_property_value284.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value285, _) => {
                                    source_ml_source2_data_property_value285.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value286, _) => {
                                    source_ml_source2_data_property_value286.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value287, _) => {
                                    source_ml_source2_data_property_value287.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value288) => {
                                    source_ml_source2_data_property_value288.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value289, _, _) => {
                                    source_ml_source2_data_property_value289.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value290, _, _) => {
                                    source_ml_source2_data_property_value290.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value291, _, _) => {
                                    source_ml_source2_data_property_value291.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value292, _, _) => {
                                    source_ml_source2_data_property_value292.clone()
                                }
                            };
                                    let ml_reference_projection149 =
                                        ml_reference_array149.borrow().len() as f64;
                                    ml_reference_projection149
                                }) >= (4096f64)))
                            {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                    Box::new(object.clone()),
                                    false,
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 112, 114, 111, 112, 101, 114,
                                        116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32,
                                        116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99,
                                        105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115,
                                        116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101,
                                        116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 111, 117, 114, 99, 101,
                                    ]),
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    ))),
                                )
                            } else {
                                {
                                    let replacement =
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            Box::new(key.clone()),
                                            (({
                                                let ml_reference_value150 = plan.borrow().clone();
                                                ml_reference_value150
                                            })
                                            .0)
                                                .0,
                                        );
                                    {
                                        let properties =
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                self::JavaScriptOrdinaryProperty,
                                            >::new(
                                            )));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_source1_temporary_values19 = match object.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value293, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value293.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value294, _, _, _) => {
                                                        source_ml_source2_data_property_value294.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value295) => {
                                                        source_ml_source2_data_property_value295.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value296, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value296.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value297, _) => {
                                                        source_ml_source2_data_property_value297.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value298, _) => {
                                                        source_ml_source2_data_property_value298.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value299, _) => {
                                                        source_ml_source2_data_property_value299.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value300, _) => {
                                                        source_ml_source2_data_property_value300.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value301) => {
                                                        source_ml_source2_data_property_value301.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value302, _, _) => {
                                                        source_ml_source2_data_property_value302.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value303, _, _) => {
                                                        source_ml_source2_data_property_value303.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value304, _, _) => {
                                                        source_ml_source2_data_property_value304.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value305, _, _) => {
                                                        source_ml_source2_data_property_value305.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_index19 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s306 = self::source_ml_source1_temporary_plan_java_script_array_ordinary_definition_loop20(at, replacement.clone(), properties.clone(), index, source_ml_source1_temporary_values19.clone(), source_ml_source1_temporary_index19.clone());
                                                        match source_ml_source2_s306.clone() {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Done => {
                                                                {
                                                                    let source_ml_source2_s307 = if (at < (0f64)) {
                                                                        {
                                                                            let source_ml_source1_temporary_plan_java_script_array_ordinary_definition_ignored22 = { let ml_method_owner151 = properties.clone(); let ml_method_argument151_0 = replacement.clone(); let ml_inserted = vec![ml_method_argument151_0]; let mut ml_method_values151 = ml_method_owner151.borrow_mut(); let ml_length = ml_method_values151.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values151.extend(ml_inserted); ml_length as f64 };
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next
                                                                    };
                                                                    match source_ml_source2_s307.clone() {
                                                                        self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next => {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field308, source_ml_source2_nominal_field309, source_ml_source2_nominal_field310, source_ml_source2_nominal_field311, _, source_ml_source2_nominal_field313, source_ml_source2_nominal_field314, source_ml_source2_nominal_field315, source_ml_source2_nominal_field316, source_ml_source2_nominal_field317) => {
                                                                                    let source_ml_source2_nominal_field309 = *source_ml_source2_nominal_field309;
                                                                                    let source_ml_source2_nominal_field316 = *source_ml_source2_nominal_field316;
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field308.clone(), Box::new(source_ml_source2_nominal_field309.clone()), source_ml_source2_nominal_field310, source_ml_source2_nominal_field311, properties.clone(), source_ml_source2_nominal_field313.clone(), source_ml_source2_nominal_field314, source_ml_source2_nominal_field315.clone(), Box::new(source_ml_source2_nominal_field316.clone()), source_ml_source2_nominal_field317.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field319, source_ml_source2_nominal_field320, source_ml_source2_nominal_field321, source_ml_source2_nominal_field322, _, source_ml_source2_nominal_field324, source_ml_source2_nominal_field325, source_ml_source2_nominal_field326) => {
                                                                                    let source_ml_source2_nominal_field320 = *source_ml_source2_nominal_field320;
                                                                                    let source_ml_source2_nominal_field324 = *source_ml_source2_nominal_field324;
                                                                                    let source_ml_source2_nominal_field325 = *source_ml_source2_nominal_field325;
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field319.clone(), Box::new(source_ml_source2_nominal_field320.clone()), source_ml_source2_nominal_field321, source_ml_source2_nominal_field322, properties.clone(), Box::new(source_ml_source2_nominal_field324.clone()), Box::new(source_ml_source2_nominal_field325.clone()), source_ml_source2_nominal_field326.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field328, source_ml_source2_nominal_field329, source_ml_source2_nominal_field330, source_ml_source2_nominal_field331, _) => {
                                                                                    let source_ml_source2_nominal_field329 = *source_ml_source2_nominal_field329;
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field328.clone(), Box::new(source_ml_source2_nominal_field329.clone()), source_ml_source2_nominal_field330, source_ml_source2_nominal_field331, properties.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field334, source_ml_source2_nominal_field335, source_ml_source2_nominal_field336, source_ml_source2_nominal_field337, _, source_ml_source2_nominal_field339, source_ml_source2_nominal_field340, source_ml_source2_nominal_field341, source_ml_source2_nominal_field342, source_ml_source2_nominal_field343, source_ml_source2_nominal_field344) => {
                                                                                    let source_ml_source2_nominal_field335 = *source_ml_source2_nominal_field335;
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field334.clone(), Box::new(source_ml_source2_nominal_field335.clone()), source_ml_source2_nominal_field336, source_ml_source2_nominal_field337, properties.clone(), source_ml_source2_nominal_field339.clone(), source_ml_source2_nominal_field340, source_ml_source2_nominal_field341.clone(), source_ml_source2_nominal_field342.clone(), source_ml_source2_nominal_field343.clone(), source_ml_source2_nominal_field344.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field346, source_ml_source2_nominal_field347, source_ml_source2_nominal_field348, source_ml_source2_nominal_field349, _, source_ml_source2_nominal_field351) => {
                                                                                    let source_ml_source2_nominal_field347 = *source_ml_source2_nominal_field347;
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field346.clone(), Box::new(source_ml_source2_nominal_field347.clone()), source_ml_source2_nominal_field348, source_ml_source2_nominal_field349, properties.clone(), source_ml_source2_nominal_field351.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field353, source_ml_source2_nominal_field354, source_ml_source2_nominal_field355, source_ml_source2_nominal_field356, _, source_ml_source2_nominal_field358) => {
                                                                                    let source_ml_source2_nominal_field354 = *source_ml_source2_nominal_field354;
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field353.clone(), Box::new(source_ml_source2_nominal_field354.clone()), source_ml_source2_nominal_field355, source_ml_source2_nominal_field356, properties.clone(), source_ml_source2_nominal_field358.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field360, source_ml_source2_nominal_field361, source_ml_source2_nominal_field362, source_ml_source2_nominal_field363, _, source_ml_source2_nominal_field365) => {
                                                                                    let source_ml_source2_nominal_field361 = *source_ml_source2_nominal_field361;
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field360.clone(), Box::new(source_ml_source2_nominal_field361.clone()), source_ml_source2_nominal_field362, source_ml_source2_nominal_field363, properties.clone(), source_ml_source2_nominal_field365.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field367, source_ml_source2_nominal_field368, source_ml_source2_nominal_field369, source_ml_source2_nominal_field370, _, source_ml_source2_nominal_field372) => {
                                                                                    let source_ml_source2_nominal_field368 = *source_ml_source2_nominal_field368;
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field367.clone(), Box::new(source_ml_source2_nominal_field368.clone()), source_ml_source2_nominal_field369, source_ml_source2_nominal_field370, properties.clone(), source_ml_source2_nominal_field372.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field374, source_ml_source2_nominal_field375, source_ml_source2_nominal_field376, source_ml_source2_nominal_field377, source_ml_source2_nominal_field378, source_ml_source2_nominal_field379, source_ml_source2_nominal_field380, source_ml_source2_nominal_field381, _) => {
                                                                                    let source_ml_source2_nominal_field375 = *source_ml_source2_nominal_field375;
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field374.clone(), Box::new(source_ml_source2_nominal_field375.clone()), source_ml_source2_nominal_field376, source_ml_source2_nominal_field377.clone(), source_ml_source2_nominal_field378.clone(), source_ml_source2_nominal_field379.clone(), source_ml_source2_nominal_field380.clone(), source_ml_source2_nominal_field381, properties.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field384, source_ml_source2_nominal_field385, source_ml_source2_nominal_field386, source_ml_source2_nominal_field387, _, source_ml_source2_nominal_field389, source_ml_source2_nominal_field390) => {
                                                                                    let source_ml_source2_nominal_field385 = *source_ml_source2_nominal_field385;
                                                                                    let source_ml_source2_nominal_field389 = *source_ml_source2_nominal_field389;
                                                                                    let source_ml_source2_nominal_field390 = *source_ml_source2_nominal_field390;
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field384.clone(), Box::new(source_ml_source2_nominal_field385.clone()), source_ml_source2_nominal_field386, source_ml_source2_nominal_field387, properties.clone(), Box::new(source_ml_source2_nominal_field389.clone()), Box::new(source_ml_source2_nominal_field390.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field392, source_ml_source2_nominal_field393, source_ml_source2_nominal_field394, source_ml_source2_nominal_field395, _, source_ml_source2_nominal_field397, source_ml_source2_nominal_field398) => {
                                                                                    let source_ml_source2_nominal_field393 = *source_ml_source2_nominal_field393;
                                                                                    let source_ml_source2_nominal_field397 = *source_ml_source2_nominal_field397;
                                                                                    let source_ml_source2_nominal_field398 = *source_ml_source2_nominal_field398;
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field392.clone(), Box::new(source_ml_source2_nominal_field393.clone()), source_ml_source2_nominal_field394, source_ml_source2_nominal_field395, properties.clone(), Box::new(source_ml_source2_nominal_field397.clone()), Box::new(source_ml_source2_nominal_field398.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field400, source_ml_source2_nominal_field401, source_ml_source2_nominal_field402, source_ml_source2_nominal_field403, _, source_ml_source2_nominal_field405, source_ml_source2_nominal_field406) => {
                                                                                    let source_ml_source2_nominal_field401 = *source_ml_source2_nominal_field401;
                                                                                    let source_ml_source2_nominal_field405 = *source_ml_source2_nominal_field405;
                                                                                    let source_ml_source2_nominal_field406 = *source_ml_source2_nominal_field406;
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field400.clone(), Box::new(source_ml_source2_nominal_field401.clone()), source_ml_source2_nominal_field402, source_ml_source2_nominal_field403, properties.clone(), Box::new(source_ml_source2_nominal_field405.clone()), Box::new(source_ml_source2_nominal_field406.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field408, source_ml_source2_nominal_field409, source_ml_source2_nominal_field410, source_ml_source2_nominal_field411, _, source_ml_source2_nominal_field413, source_ml_source2_nominal_field414) => {
                                                                                    let source_ml_source2_nominal_field409 = *source_ml_source2_nominal_field409;
                                                                                    let source_ml_source2_nominal_field413 = *source_ml_source2_nominal_field413;
                                                                                    let source_ml_source2_nominal_field414 = *source_ml_source2_nominal_field414;
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field408.clone(), Box::new(source_ml_source2_nominal_field409.clone()), source_ml_source2_nominal_field410, source_ml_source2_nominal_field411, properties.clone(), Box::new(source_ml_source2_nominal_field413.clone()), Box::new(source_ml_source2_nominal_field414.clone()))
                                                                                }
                                                                            }), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), (({ let ml_reference_value152 = plan.borrow().clone(); ml_reference_value152 }).0).3)
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
