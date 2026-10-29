// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::apply_java_script_descriptor_field;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_field;
use super::read_java_script_literal_property_index;
use super::test_java_script_descriptor_field_presence;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result {
    MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(
        Vec<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptSemanticValue>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop24(
    positions: &[f64],
    mut position: f64,
    mut at: f64,
) -> f64 {
    loop {
        if (at < (positions.len() as f64))
            && (((self::ml_array::optional_at(positions, self::ml_array::number_index(at)))
                .unwrap_or_else(|| -1f64))
                < position)
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop23(
    mut numeric: Vec<self::JavaScriptSemanticValue>,
    mut positions: Vec<f64>,
    mut text: Vec<self::JavaScriptSemanticValue>,
    mut symbols: Vec<self::JavaScriptSemanticValue>,
    mut source_ml_temporary_values22: Vec<self::JavaScriptOrdinaryProperty>,
    mut source_ml_temporary_index22: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result {
    loop {
        return if source_ml_temporary_index22
            < (self::ml::Big::from_u128(source_ml_temporary_values22.len() as u128))
        {
            {
                let property = self::ml_array::at(
                    &source_ml_temporary_values22,
                    source_ml_temporary_index22.to_index(),
                );
                {
                    let key = match property {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value946,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value946 =
                                *source_ml_source1_data_property_value946;
                            source_ml_source1_data_property_value946.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            let symbols_2 = {
                                let source_ml_source1_concat_value953 = symbols;
                                {
                                    let source_ml_source1_concat_value954 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value953,
                                        ),
                                        source_ml_source1_concat_value954,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_2 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_2, source_ml_temporary_index22_2);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Null => {
                            let symbols_3 = {
                                let source_ml_source1_concat_value955 = symbols;
                                {
                                    let source_ml_source1_concat_value956 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value955,
                                        ),
                                        source_ml_source1_concat_value956,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_3 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_3, source_ml_temporary_index22_3);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f947) => {
                            let symbols_4 = {
                                let source_ml_source1_concat_value957 = symbols;
                                {
                                    let source_ml_source1_concat_value958 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value957,
                                        ),
                                        source_ml_source1_concat_value958,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_4 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_4, source_ml_temporary_index22_4);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f948) => {
                            let symbols_5 = {
                                let source_ml_source1_concat_value959 = symbols;
                                {
                                    let source_ml_source1_concat_value960 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value959,
                                        ),
                                        source_ml_source1_concat_value960,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_5 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_5, source_ml_temporary_index22_5);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f949) => {
                            let symbols_6 = {
                                let source_ml_source1_concat_value961 = symbols;
                                {
                                    let source_ml_source1_concat_value962 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value961,
                                        ),
                                        source_ml_source1_concat_value962,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_6 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_6, source_ml_temporary_index22_6);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units) => {
                            let position = self::read_java_script_literal_property_index(&(units));
                            if position < 0f64 {
                                {
                                    let text_2 = {
                                        let source_ml_source1_concat_value963 = text;
                                        {
                                            let source_ml_source1_concat_value964 =
                                                vec![key.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::JavaScriptSemanticValue>::new(),
                                                    source_ml_source1_concat_value963,
                                                ),
                                                source_ml_source1_concat_value964,
                                            )
                                        }
                                    };
                                    {
                                        let source_ml_temporary_index22_7 =
                                            source_ml_temporary_index22
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (text, source_ml_temporary_index22) =
                                                (text_2, source_ml_temporary_index22_7);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let at = 0f64;
                                    {
                                        let at_2 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop24(&(positions), position, at);
                                        {
                                            let numeric_2 = {
                                                let source_ml_source1_concat_value965 =
                                                    self::ml_array::slice(&numeric, 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value966 =
                                                        vec![key.clone()];
                                                    {
                                                        let source_ml_source1_concat_value967 =
                                                            self::ml_array::slice(
                                                                &numeric,
                                                                at_2,
                                                                f64::INFINITY,
                                                            );
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value965), source_ml_source1_concat_value966), source_ml_source1_concat_value967)
                                                    }
                                                }
                                            };
                                            {
                                                let positions_2 = {
                                                    let source_ml_source1_concat_value968 =
                                                        self::ml_array::slice(
                                                            &positions, 0f64, at_2,
                                                        );
                                                    {
                                                        let source_ml_source1_concat_value969 =
                                                            vec![position];
                                                        {
                                                            let source_ml_source1_concat_value970 =
                                                                self::ml_array::slice(
                                                                    &positions,
                                                                    at_2,
                                                                    f64::INFINITY,
                                                                );
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value968), source_ml_source1_concat_value969), source_ml_source1_concat_value970)
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index22_8 =
                                                        source_ml_temporary_index22
                                                            .add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (
                                                            numeric,
                                                            positions,
                                                            source_ml_temporary_index22,
                                                        ) = (
                                                            numeric_2,
                                                            positions_2,
                                                            source_ml_temporary_index22_8,
                                                        );
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f950) => {
                            let symbols_7 = {
                                let source_ml_source1_concat_value971 = symbols;
                                {
                                    let source_ml_source1_concat_value972 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value971,
                                        ),
                                        source_ml_source1_concat_value972,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_9 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_7, source_ml_temporary_index22_9);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f951,
                            source_ml_source1_f952,
                        ) => {
                            let symbols_8 = {
                                let source_ml_source1_concat_value973 = symbols;
                                {
                                    let source_ml_source1_concat_value974 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value973,
                                        ),
                                        source_ml_source1_concat_value974,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_10 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_8, source_ml_temporary_index22_10);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric, text, symbols)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_insert_java_script_ordinary_object_loop26(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_temporary_values25: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index25: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index25
            < (self::ml::Big::from_u128(source_ml_temporary_values25.len() as u128))
        {
            {
                let object = self::ml_array::at(
                    &source_ml_temporary_values25,
                    source_ml_temporary_index25.to_index(),
                );
                if (match object {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        source_ml_source1_data_property_value976,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value976.clone(),
                }) < identity
                {
                    {
                        let at_2 = at + 1f64;
                        {
                            let source_ml_temporary_index25_2 =
                                source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index25) =
                                    (at_2, source_ml_temporary_index25_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index25_3 =
                            source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index25 = source_ml_temporary_index25_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { at };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop28(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_temporary_values27: Vec<self::ml::Big>,
    mut source_ml_temporary_index27: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index27
            < (self::ml::Big::from_u128(source_ml_temporary_values27.len() as u128))
        {
            {
                let symbol = self::ml_array::at(
                    &source_ml_temporary_values27,
                    source_ml_temporary_index27.to_index(),
                );
                if symbol < identity {
                    {
                        let at_2 = at + 1f64;
                        {
                            let source_ml_temporary_index27_2 =
                                source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index27) =
                                    (at_2, source_ml_temporary_index27_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index27_3 =
                            source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index27 = source_ml_temporary_index27_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result {
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Done,
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop29(
    mut state: self::JavaScriptDescriptorConversionState,
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result {
    loop {
        return if index < 6f64 {
            if self::test_java_script_descriptor_field_presence(
                match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        _,
                        source_ml_source1_data_property_value977,
                        _,
                    ) => source_ml_source1_data_property_value977.clone(),
                },
                index,
            ) {
                self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Return(String::from("unprocessed descriptor attributes cannot be fabricated in a continuation"))
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryStepJavaScriptDescriptorConversionLoop30Result {
    MlTemporaryStepJavaScriptDescriptorConversionLoop30Done(
        Box<self::JavaScriptDescriptorConversionState>,
    ),
    MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(
        Box<self::JavaScriptDescriptorConversionTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_step_java_script_descriptor_conversion_loop30(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut current: self::JavaScriptDescriptorConversionState,
) -> self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result {
    loop {
        return if (match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                source_ml_source1_data_property_value979,
                _,
                _,
            ) => source_ml_source1_data_property_value979,
        }) < 6f64
        {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value980,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value980,
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                source_ml_source1_data_property_value981,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source1_data_property_value981 =
                                    *source_ml_source1_data_property_value981;
                                source_ml_source1_data_property_value981.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f982) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f983) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f984) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f985) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f986) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f987) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value988, _, _) => {
                                                    source_ml_source1_data_property_value988
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value989, _) => {
                                                    source_ml_source1_data_property_value989.clone()
                                                }
                                            }, false);
                                            {
                                                current = current_2;
                                                continue;
                                            }
                                        }
                                    }
{
                                        {
                                            let plan = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                            {
let ml_library_inequality_2 = ((plan.clone()).3) != "" ;
if ml_library_inequality_2 {
                                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else if ((plan.clone()).0) == "call" {
                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value990, _, _) => {
                                                            source_ml_source1_data_property_value990
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value991, _) => {
                                                            source_ml_source1_data_property_value991.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
let ml_library_inequality_1 = ((plan.clone()).0) != "value" ;
if ml_library_inequality_1 {
                                                        self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("ordinary Get requires its actual value or pending call"))))
                                                    } else {
                                                        {
                                                            let applied = self::apply_java_script_descriptor_field(current.clone(), (plan.clone()).5);
                                                            match applied.clone() {
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(current_3) => {
                                                                    let current_3 = *current_3;
                                                                    {
                                                                        current = current_3.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f992, source_ml_source1_f993, source_ml_source1_f994, source_ml_source1_f995) => {
                                                                    let source_ml_source1_f992 = *source_ml_source1_f992;
                                                                    let source_ml_source1_f993 = *source_ml_source1_f993;
                                                                    let source_ml_source1_f994 = *source_ml_source1_f994;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f996) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f997) => {
                                                                    let source_ml_source1_f997 = *source_ml_source1_f997;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f998, source_ml_source1_f999) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f1000) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
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
        } else {
            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Done(Box::new(current.clone()))
        };
    }
}
