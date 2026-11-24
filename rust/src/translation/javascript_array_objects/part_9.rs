// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Result;
use super::ml;
use super::ml_array;
use super::read_java_script_array_index;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_ordinary_definition_loop16(
    mut at: f64,
    mut replacement: self::JavaScriptOrdinaryProperty,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut index: f64,
    mut source_ml_temporary_values15: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index15: self::ml::Big,
) -> self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Result {
    loop {
        if source_ml_temporary_index15
            < ({
                let ml_reference_array175 = source_ml_temporary_values15.clone();
                self::ml::Big::from_u128(ml_reference_array175.borrow().len() as u128)
            })
        {
            {
                let entry = {
                    let source_ml_source1_array_read_value615 =
                        source_ml_temporary_values15.clone();
                    {
                        let source_ml_source1_array_read_index616 =
                            source_ml_temporary_index15.clone();
                        {
                            let ml_reference_array176 = source_ml_source1_array_read_value615;
                            let ml_reference_position176 = source_ml_source1_array_read_index616;
                            self::ml_array::at(
                                &*ml_reference_array176.borrow(),
                                ml_reference_position176.to_index(),
                            )
                        }
                    }
                };
                {
                    let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored19 = {
                        let ml_method_owner177 = properties.clone();
                        let ml_method_argument177_0 = if index == at {
                            replacement.clone()
                        } else {
                            entry
                        };
                        let ml_inserted = vec![ml_method_argument177_0];
                        let mut ml_method_values177 = ml_method_owner177.borrow_mut();
                        let ml_length = ml_method_values177
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values177.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_temporary_index15_2 =
                                source_ml_temporary_index15.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index15) =
                                    (index_2, source_ml_temporary_index15_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_length_definition_loop22(
    mut unsigned_length: f64,
    mut blocked: f64,
    mut source_ml_temporary_values20: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index20: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index20
            < ({
                let ml_reference_array178 = source_ml_temporary_values20.clone();
                self::ml::Big::from_u128(ml_reference_array178.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let source_ml_source1_array_read_value617 =
                        source_ml_temporary_values20.clone();
                    {
                        let source_ml_source1_array_read_index618 =
                            source_ml_temporary_index20.clone();
                        {
                            let ml_reference_array179 = source_ml_source1_array_read_value617;
                            let ml_reference_position179 = source_ml_source1_array_read_index618;
                            self::ml_array::at(
                                &*ml_reference_array179.borrow(),
                                ml_reference_position179.to_index(),
                            )
                        }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value619,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value619 =
                                *source_ml_source1_data_property_value619;
                            source_ml_source1_data_property_value619.clone()
                        }
                    });
                    if ((index >= unsigned_length)
                        && !(({
                            (match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source1_data_property_value620,
                                ) => source_ml_source1_data_property_value620.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .0))
                        && (index > blocked)
                    {
                        {
                            let blocked_2 = index;
                            {
                                let source_ml_temporary_index20_2 =
                                    source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                                {
                                    (blocked, source_ml_temporary_index20) =
                                        (blocked_2, source_ml_temporary_index20_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index20_3 =
                                source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index20 = source_ml_temporary_index20_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { blocked };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Result {
    MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_length_definition_loop23(
    mut new_writable: bool,
    mut final_length: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut source_ml_temporary_values21: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index21: self::ml::Big,
) -> self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Result {
    loop {
        if source_ml_temporary_index21
            < ({
                let ml_reference_array181 = source_ml_temporary_values21.clone();
                self::ml::Big::from_u128(ml_reference_array181.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let source_ml_source1_array_read_value621 =
                        source_ml_temporary_values21.clone();
                    {
                        let source_ml_source1_array_read_index622 =
                            source_ml_temporary_index21.clone();
                        {
                            let ml_reference_array182 = source_ml_source1_array_read_value621;
                            let ml_reference_position182 = source_ml_source1_array_read_index622;
                            self::ml_array::at(
                                &*ml_reference_array182.borrow(),
                                ml_reference_position182.to_index(),
                            )
                        }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value623,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value623 =
                                *source_ml_source1_data_property_value623;
                            source_ml_source1_data_property_value623.clone()
                        }
                    });
                    if (index < 0f64) || (index < final_length) {
                        {
                            let source_ml_temporary_plan_java_script_array_length_definition_ignored24 = {
                                let ml_method_owner183 = properties.clone();
                                let ml_method_argument183_0 =
                                    if self::test_java_script_array_length_key(
                                        match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source1_data_property_value624,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value624 =
                                                    *source_ml_source1_data_property_value624;
                                                source_ml_source1_data_property_value624.clone()
                                            }
                                        },
                                    ) {
                                        {
                                            let source_ml_source1_nominal_spread641 =
                                                property.clone();
                                            {
                                                let source_ml_source1_nominal_field642 = match source_ml_source1_nominal_spread641.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value643, _) => {
                                                let source_ml_source1_data_property_value643 = *source_ml_source1_data_property_value643;
                                                source_ml_source1_data_property_value643.clone()
                                            }
                                        };
                                                {
                                                    let source_ml_source1_nominal_field644 = match source_ml_source1_nominal_spread641.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value645) => {
                                                    source_ml_source1_data_property_value645.clone()
                                                }
                                            };
                                                    {
                                                        let source_ml_source1_nominal_override646 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new({
                                                                    let source_ml_source1_spread626 = {
                                                                        (match property.clone() {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value625) => {
                                                            source_ml_source1_data_property_value625.clone()
                                                        }
                                                    }).borrow().clone()
                                                                    };
                                                                    {
                                                                        let source_ml_source1_spread_field627 = (source_ml_source1_spread626.clone()).0;
                                                                        {
                                                                            let source_ml_source1_spread_field628 = (source_ml_source1_spread626.clone()).1;
                                                                            {
                                                                                let source_ml_source1_spread_field629 = (source_ml_source1_spread626.clone()).2;
                                                                                {
                                                                                    let source_ml_source1_spread_field630 = (source_ml_source1_spread626.clone()).3;
                                                                                    {
                                                                                        let source_ml_source1_spread_field631 = (source_ml_source1_spread626.clone()).4;
                                                                                        {
                                                                                            let source_ml_source1_spread_field632 = (source_ml_source1_spread626.clone()).5;
                                                                                            {
                                                                                                let source_ml_source1_spread_field633 = (source_ml_source1_spread626.clone()).6;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field634 = (source_ml_source1_spread626.clone()).7;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field635 = (source_ml_source1_spread626.clone()).8;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field636 = (source_ml_source1_spread626.clone()).9;
                                                                                                            {
                                                                                                                let source_ml_source1_spread_field637 = (source_ml_source1_spread626.clone()).10;
                                                                                                                {
                                                                                                                    let source_ml_source1_spread_field638 = (source_ml_source1_spread626.clone()).11;
                                                                                                                    {
                                                                                                                        let source_ml_source1_field639 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                                        {
                                                                                                                            let source_ml_source1_field640 = new_writable;
                                                                                                                            (source_ml_source1_spread_field627, source_ml_source1_spread_field628, source_ml_source1_spread_field629, source_ml_source1_spread_field630, source_ml_source1_spread_field631, source_ml_source1_spread_field632, source_ml_source1_spread_field633, source_ml_source1_spread_field634, source_ml_source1_field639, source_ml_source1_spread_field636, source_ml_source1_field640, source_ml_source1_spread_field638)
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
                                                            );
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field642), source_ml_source1_nominal_override646)
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    };
                                let ml_inserted = vec![ml_method_argument183_0];
                                let mut ml_method_values183 = ml_method_owner183.borrow_mut();
                                let ml_length = ml_method_values183
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values183.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_temporary_index21_2 =
                                    source_ml_temporary_index21.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index21 = source_ml_temporary_index21_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index21_3 =
                                source_ml_temporary_index21.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index21 = source_ml_temporary_index21_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Result {
    MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_property_definition_loop26(
    mut index: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut source_ml_temporary_values25: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index25: self::ml::Big,
) -> self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Result {
    loop {
        if source_ml_temporary_index25
            < ({
                let ml_reference_array185 = source_ml_temporary_values25.clone();
                self::ml::Big::from_u128(ml_reference_array185.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let source_ml_source1_array_read_value647 =
                        source_ml_temporary_values25.clone();
                    {
                        let source_ml_source1_array_read_index648 =
                            source_ml_temporary_index25.clone();
                        {
                            let ml_reference_array186 = source_ml_source1_array_read_value647;
                            let ml_reference_position186 = source_ml_source1_array_read_index648;
                            self::ml_array::at(
                                &*ml_reference_array186.borrow(),
                                ml_reference_position186.to_index(),
                            )
                        }
                    }
                };
                {
                    let source_ml_temporary_plan_java_script_array_property_definition_ignored27 = {
                        let ml_method_owner187 = properties.clone();
                        let ml_method_argument187_0 = if self::test_java_script_array_length_key(
                            match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source1_data_property_value649,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value649 =
                                        *source_ml_source1_data_property_value649;
                                    source_ml_source1_data_property_value649.clone()
                                }
                            },
                        ) {
                            {
                                let source_ml_source1_nominal_spread665 = property.clone();
                                {
                                    let source_ml_source1_nominal_field666 =
                                        match source_ml_source1_nominal_spread665.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source1_data_property_value667,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value667 =
                                                    *source_ml_source1_data_property_value667;
                                                source_ml_source1_data_property_value667.clone()
                                            }
                                        };
                                    {
                                        let source_ml_source1_nominal_field668 = match source_ml_source1_nominal_spread665.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value669) => {
                                            source_ml_source1_data_property_value669.clone()
                                        }
                                    };
                                        {
                                            let source_ml_source1_nominal_override670 =
                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_spread651 = {
                                                        (match property.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value650) => {
                                                    source_ml_source1_data_property_value650.clone()
                                                }
                                            }).borrow().clone()
                                                    };
                                                    {
                                                        let source_ml_source1_spread_field652 =
                                                            (source_ml_source1_spread651.clone()).0;
                                                        {
                                                            let source_ml_source1_spread_field653 =
                                                                (source_ml_source1_spread651
                                                                    .clone())
                                                                .1;
                                                            {
                                                                let source_ml_source1_spread_field654 =
                                                                    (source_ml_source1_spread651
                                                                        .clone())
                                                                    .2;
                                                                {
                                                                    let source_ml_source1_spread_field655 = (source_ml_source1_spread651.clone()).3;
                                                                    {
                                                                        let source_ml_source1_spread_field656 = (source_ml_source1_spread651.clone()).4;
                                                                        {
                                                                            let source_ml_source1_spread_field657 = (source_ml_source1_spread651.clone()).5;
                                                                            {
                                                                                let source_ml_source1_spread_field658 = (source_ml_source1_spread651.clone()).6;
                                                                                {
                                                                                    let source_ml_source1_spread_field659 = (source_ml_source1_spread651.clone()).7;
                                                                                    {
                                                                                        let source_ml_source1_spread_field660 = (source_ml_source1_spread651.clone()).8;
                                                                                        {
                                                                                            let source_ml_source1_spread_field661 = (source_ml_source1_spread651.clone()).9;
                                                                                            {
                                                                                                let source_ml_source1_spread_field662 = (source_ml_source1_spread651.clone()).10;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field663 = (source_ml_source1_spread651.clone()).11;
                                                                                                    {
                                                                                                        let source_ml_source1_field664 = self::JavaScriptSemanticValue::Number(index + 1f64);
                                                                                                        (source_ml_source1_spread_field652, source_ml_source1_spread_field653, source_ml_source1_spread_field654, source_ml_source1_spread_field655, source_ml_source1_spread_field656, source_ml_source1_spread_field657, source_ml_source1_spread_field658, source_ml_source1_spread_field659, source_ml_source1_field664, source_ml_source1_spread_field661, source_ml_source1_spread_field662, source_ml_source1_spread_field663)
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
                                                }));
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                Box::new(source_ml_source1_nominal_field666),
                                                source_ml_source1_nominal_override670,
                                            )
                                        }
                                    }
                                }
                            }
                        } else {
                            property.clone()
                        };
                        let ml_inserted = vec![ml_method_argument187_0];
                        let mut ml_method_values187 = ml_method_owner187.borrow_mut();
                        let ml_length = ml_method_values187
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values187.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_index25_2 =
                            source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index25 = source_ml_temporary_index25_2;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Done
        };
    }
}
