// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_ordinary_definition(
    object: self::JavaScriptOrdinaryObject,
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
                self::JavaScriptOrdinaryObject::ClosureObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value762,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value762.clone(),
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value763,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value763.clone(),
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value764,
                ) => source_ml_source2_data_property_value764.clone(),
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value765,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value765.clone(),
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value766,
                    _,
                ) => source_ml_source2_data_property_value766.clone(),
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value767,
                    _,
                ) => source_ml_source2_data_property_value767.clone(),
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value768,
                    _,
                ) => source_ml_source2_data_property_value768.clone(),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value769,
                    _,
                ) => source_ml_source2_data_property_value769.clone(),
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value770,
                ) => source_ml_source2_data_property_value770.clone(),
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value771,
                    _,
                    _,
                ) => source_ml_source2_data_property_value771.clone(),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value772,
                    _,
                    _,
                ) => source_ml_source2_data_property_value772.clone(),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value773,
                    _,
                    _,
                ) => source_ml_source2_data_property_value773.clone(),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value774,
                    _,
                    _,
                ) => source_ml_source2_data_property_value774.clone(),
            },
            key.clone(),
        );
        {
            let property = {
                let source_ml_source2_array_read_value788 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value775,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value775.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value776,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value776.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value777,
                    ) => source_ml_source2_data_property_value777.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value778,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value778.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value779,
                        _,
                    ) => source_ml_source2_data_property_value779.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value780,
                        _,
                    ) => source_ml_source2_data_property_value780.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value781,
                        _,
                    ) => source_ml_source2_data_property_value781.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value782,
                        _,
                    ) => source_ml_source2_data_property_value782.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value783,
                    ) => source_ml_source2_data_property_value783.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value784,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value784.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value785,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value785.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value786,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value786.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value787,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value787.clone(),
                };
                {
                    let ml_reference_array110 = source_ml_source2_array_read_value788;
                    let ml_reference_position110 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array110.borrow(),
                        self::ml_array::number_index(ml_reference_position110),
                    )
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
                            source_ml_source2_data_property_value790,
                        ) => source_ml_source2_data_property_value790.clone(),
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
                                source_ml_source2_data_property_value791,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value791,
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                source_ml_source2_data_property_value792,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value792,
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                source_ml_source2_data_property_value793,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value793,
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                source_ml_source2_data_property_value794,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value794,
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                source_ml_source2_data_property_value795,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value795,
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                source_ml_source2_data_property_value796,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value796,
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                source_ml_source2_data_property_value797,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value797,
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                source_ml_source2_data_property_value798,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value798,
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                source_ml_source2_data_property_value799,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value799,
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                source_ml_source2_data_property_value800,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value800,
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                source_ml_source2_data_property_value801,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value801,
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                source_ml_source2_data_property_value802,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value802,
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                source_ml_source2_data_property_value803,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value803,
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
                            let ml_reference_array114 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value817,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value817.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value818,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value818.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value819,
                                ) => source_ml_source2_data_property_value819.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value820,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value820.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value821,
                                    _,
                                ) => source_ml_source2_data_property_value821.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value822,
                                    _,
                                ) => source_ml_source2_data_property_value822.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value823,
                                    _,
                                ) => source_ml_source2_data_property_value823.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value824,
                                    _,
                                ) => source_ml_source2_data_property_value824.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value825,
                                ) => source_ml_source2_data_property_value825.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value826,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value826.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value827,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value827.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value828,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value828.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value829,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value829.clone(),
                            };
                            ml_reference_array114.borrow().len() as f64
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
                                        let source_ml_source1_temporary_values19 = match object.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value830, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value830.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value831, _, _, _) => {
                                                        source_ml_source2_data_property_value831.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value832) => {
                                                        source_ml_source2_data_property_value832.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value833, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value833.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value834, _) => {
                                                        source_ml_source2_data_property_value834.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value835, _) => {
                                                        source_ml_source2_data_property_value835.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value836, _) => {
                                                        source_ml_source2_data_property_value836.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value837, _) => {
                                                        source_ml_source2_data_property_value837.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value838) => {
                                                        source_ml_source2_data_property_value838.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value839, _, _) => {
                                                        source_ml_source2_data_property_value839.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value840, _, _) => {
                                                        source_ml_source2_data_property_value840.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value841, _, _) => {
                                                        source_ml_source2_data_property_value841.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value842, _, _) => {
                                                        source_ml_source2_data_property_value842.clone()
                                                    }
                                                };
                                        {
                                            let source_ml_source1_temporary_index19 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s843 = self::source_ml_source1_temporary_plan_java_script_array_ordinary_definition_loop20(at, replacement.clone(), properties.clone(), index, source_ml_source1_temporary_values19, source_ml_source1_temporary_index19);
                                                match source_ml_source2_s843 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Result::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop20Done => {
                                                                {
                                                                    let source_ml_source2_s844 = if at < 0f64 {
                                                                        {
                                                                            let source_ml_source1_temporary_plan_java_script_array_ordinary_definition_ignored22 = { let ml_method_owner116 = properties.clone(); let ml_method_argument116_0 = replacement.clone(); let ml_inserted = vec![ml_method_argument116_0]; let mut ml_method_values116 = ml_method_owner116.borrow_mut(); let ml_length = ml_method_values116.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values116.extend(ml_inserted); ml_length as f64 };
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next
                                                                    };
                                                                    match source_ml_source2_s844 {
                                                                        self::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21::MlSource1TemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin21Next => {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field845, source_ml_source2_nominal_field846, source_ml_source2_nominal_field847, source_ml_source2_nominal_field848, _, source_ml_source2_nominal_field850, source_ml_source2_nominal_field851, source_ml_source2_nominal_field852, source_ml_source2_nominal_field853, source_ml_source2_nominal_field854) => {
                                                                                    let source_ml_source2_nominal_field846 = *source_ml_source2_nominal_field846;
                                                                                    let source_ml_source2_nominal_field853 = *source_ml_source2_nominal_field853;
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field845.clone(), Box::new(source_ml_source2_nominal_field846.clone()), source_ml_source2_nominal_field847, source_ml_source2_nominal_field848, properties.clone(), source_ml_source2_nominal_field850.clone(), source_ml_source2_nominal_field851, source_ml_source2_nominal_field852.clone(), Box::new(source_ml_source2_nominal_field853.clone()), source_ml_source2_nominal_field854.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field856, source_ml_source2_nominal_field857, source_ml_source2_nominal_field858, source_ml_source2_nominal_field859, _, source_ml_source2_nominal_field861, source_ml_source2_nominal_field862, source_ml_source2_nominal_field863) => {
                                                                                    let source_ml_source2_nominal_field857 = *source_ml_source2_nominal_field857;
                                                                                    let source_ml_source2_nominal_field861 = *source_ml_source2_nominal_field861;
                                                                                    let source_ml_source2_nominal_field862 = *source_ml_source2_nominal_field862;
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field856.clone(), Box::new(source_ml_source2_nominal_field857.clone()), source_ml_source2_nominal_field858, source_ml_source2_nominal_field859, properties.clone(), Box::new(source_ml_source2_nominal_field861.clone()), Box::new(source_ml_source2_nominal_field862.clone()), source_ml_source2_nominal_field863.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field865, source_ml_source2_nominal_field866, source_ml_source2_nominal_field867, source_ml_source2_nominal_field868, _) => {
                                                                                    let source_ml_source2_nominal_field866 = *source_ml_source2_nominal_field866;
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field865.clone(), Box::new(source_ml_source2_nominal_field866.clone()), source_ml_source2_nominal_field867, source_ml_source2_nominal_field868, properties.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field871, source_ml_source2_nominal_field872, source_ml_source2_nominal_field873, source_ml_source2_nominal_field874, _, source_ml_source2_nominal_field876, source_ml_source2_nominal_field877, source_ml_source2_nominal_field878, source_ml_source2_nominal_field879, source_ml_source2_nominal_field880, source_ml_source2_nominal_field881) => {
                                                                                    let source_ml_source2_nominal_field872 = *source_ml_source2_nominal_field872;
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field871.clone(), Box::new(source_ml_source2_nominal_field872.clone()), source_ml_source2_nominal_field873, source_ml_source2_nominal_field874, properties.clone(), source_ml_source2_nominal_field876.clone(), source_ml_source2_nominal_field877, source_ml_source2_nominal_field878.clone(), source_ml_source2_nominal_field879.clone(), source_ml_source2_nominal_field880.clone(), source_ml_source2_nominal_field881.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field883, source_ml_source2_nominal_field884, source_ml_source2_nominal_field885, source_ml_source2_nominal_field886, _, source_ml_source2_nominal_field888) => {
                                                                                    let source_ml_source2_nominal_field884 = *source_ml_source2_nominal_field884;
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field883.clone(), Box::new(source_ml_source2_nominal_field884.clone()), source_ml_source2_nominal_field885, source_ml_source2_nominal_field886, properties.clone(), source_ml_source2_nominal_field888.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field890, source_ml_source2_nominal_field891, source_ml_source2_nominal_field892, source_ml_source2_nominal_field893, _, source_ml_source2_nominal_field895) => {
                                                                                    let source_ml_source2_nominal_field891 = *source_ml_source2_nominal_field891;
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field890.clone(), Box::new(source_ml_source2_nominal_field891.clone()), source_ml_source2_nominal_field892, source_ml_source2_nominal_field893, properties.clone(), source_ml_source2_nominal_field895.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field897, source_ml_source2_nominal_field898, source_ml_source2_nominal_field899, source_ml_source2_nominal_field900, _, source_ml_source2_nominal_field902) => {
                                                                                    let source_ml_source2_nominal_field898 = *source_ml_source2_nominal_field898;
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field897.clone(), Box::new(source_ml_source2_nominal_field898.clone()), source_ml_source2_nominal_field899, source_ml_source2_nominal_field900, properties.clone(), source_ml_source2_nominal_field902.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field904, source_ml_source2_nominal_field905, source_ml_source2_nominal_field906, source_ml_source2_nominal_field907, _, source_ml_source2_nominal_field909) => {
                                                                                    let source_ml_source2_nominal_field905 = *source_ml_source2_nominal_field905;
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field904.clone(), Box::new(source_ml_source2_nominal_field905.clone()), source_ml_source2_nominal_field906, source_ml_source2_nominal_field907, properties.clone(), source_ml_source2_nominal_field909.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field911, source_ml_source2_nominal_field912, source_ml_source2_nominal_field913, source_ml_source2_nominal_field914, source_ml_source2_nominal_field915, source_ml_source2_nominal_field916, source_ml_source2_nominal_field917, source_ml_source2_nominal_field918, _) => {
                                                                                    let source_ml_source2_nominal_field912 = *source_ml_source2_nominal_field912;
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field911.clone(), Box::new(source_ml_source2_nominal_field912.clone()), source_ml_source2_nominal_field913, source_ml_source2_nominal_field914.clone(), source_ml_source2_nominal_field915.clone(), source_ml_source2_nominal_field916.clone(), source_ml_source2_nominal_field917.clone(), source_ml_source2_nominal_field918, properties.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field921, source_ml_source2_nominal_field922, source_ml_source2_nominal_field923, source_ml_source2_nominal_field924, _, source_ml_source2_nominal_field926, source_ml_source2_nominal_field927) => {
                                                                                    let source_ml_source2_nominal_field922 = *source_ml_source2_nominal_field922;
                                                                                    let source_ml_source2_nominal_field926 = *source_ml_source2_nominal_field926;
                                                                                    let source_ml_source2_nominal_field927 = *source_ml_source2_nominal_field927;
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field921.clone(), Box::new(source_ml_source2_nominal_field922.clone()), source_ml_source2_nominal_field923, source_ml_source2_nominal_field924, properties.clone(), Box::new(source_ml_source2_nominal_field926.clone()), Box::new(source_ml_source2_nominal_field927.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field929, source_ml_source2_nominal_field930, source_ml_source2_nominal_field931, source_ml_source2_nominal_field932, _, source_ml_source2_nominal_field934, source_ml_source2_nominal_field935) => {
                                                                                    let source_ml_source2_nominal_field930 = *source_ml_source2_nominal_field930;
                                                                                    let source_ml_source2_nominal_field934 = *source_ml_source2_nominal_field934;
                                                                                    let source_ml_source2_nominal_field935 = *source_ml_source2_nominal_field935;
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field929.clone(), Box::new(source_ml_source2_nominal_field930.clone()), source_ml_source2_nominal_field931, source_ml_source2_nominal_field932, properties.clone(), Box::new(source_ml_source2_nominal_field934.clone()), Box::new(source_ml_source2_nominal_field935.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field937, source_ml_source2_nominal_field938, source_ml_source2_nominal_field939, source_ml_source2_nominal_field940, _, source_ml_source2_nominal_field942, source_ml_source2_nominal_field943) => {
                                                                                    let source_ml_source2_nominal_field938 = *source_ml_source2_nominal_field938;
                                                                                    let source_ml_source2_nominal_field942 = *source_ml_source2_nominal_field942;
                                                                                    let source_ml_source2_nominal_field943 = *source_ml_source2_nominal_field943;
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field937.clone(), Box::new(source_ml_source2_nominal_field938.clone()), source_ml_source2_nominal_field939, source_ml_source2_nominal_field940, properties.clone(), Box::new(source_ml_source2_nominal_field942.clone()), Box::new(source_ml_source2_nominal_field943.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field945, source_ml_source2_nominal_field946, source_ml_source2_nominal_field947, source_ml_source2_nominal_field948, _, source_ml_source2_nominal_field950, source_ml_source2_nominal_field951) => {
                                                                                    let source_ml_source2_nominal_field946 = *source_ml_source2_nominal_field946;
                                                                                    let source_ml_source2_nominal_field950 = *source_ml_source2_nominal_field950;
                                                                                    let source_ml_source2_nominal_field951 = *source_ml_source2_nominal_field951;
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field945.clone(), Box::new(source_ml_source2_nominal_field946.clone()), source_ml_source2_nominal_field947, source_ml_source2_nominal_field948, properties.clone(), Box::new(source_ml_source2_nominal_field950.clone()), Box::new(source_ml_source2_nominal_field951.clone()))
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
