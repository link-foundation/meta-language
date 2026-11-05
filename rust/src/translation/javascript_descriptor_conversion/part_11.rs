// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::apply_java_script_descriptor_field;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_field;
use super::test_java_script_descriptor_field_presence;

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
                        source_ml_source1_data_property_value1140,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1140.clone(),
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
                        source_ml_source1_data_property_value1141,
                        _,
                    ) => source_ml_source1_data_property_value1141.clone(),
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
                source_ml_source1_data_property_value1143,
                _,
                _,
            ) => source_ml_source1_data_property_value1143,
        }) < 6f64
        {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value1144,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1144,
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                source_ml_source1_data_property_value1145,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source1_data_property_value1145 =
                                    *source_ml_source1_data_property_value1145;
                                source_ml_source1_data_property_value1145.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1146) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1147) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1148) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1149) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1150) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from("descriptor conversion retains its actual Object receiver"))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1151) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1152, _, _) => {
                                                    source_ml_source1_data_property_value1152
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1153, _) => {
                                                    source_ml_source1_data_property_value1153.clone()
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
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1154, _, _) => {
                                                            source_ml_source1_data_property_value1154
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1155, _) => {
                                                            source_ml_source1_data_property_value1155.clone()
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
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f1156, source_ml_source1_f1157, source_ml_source1_f1158, source_ml_source1_f1159) => {
                                                                    let source_ml_source1_f1156 = *source_ml_source1_f1156;
                                                                    let source_ml_source1_f1157 = *source_ml_source1_f1157;
                                                                    let source_ml_source1_f1158 = *source_ml_source1_f1158;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f1160) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f1161) => {
                                                                    let source_ml_source1_f1161 = *source_ml_source1_f1161;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f1162, source_ml_source1_f1163) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f1164) => {
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
