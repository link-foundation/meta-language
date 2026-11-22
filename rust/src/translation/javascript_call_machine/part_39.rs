// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_array;
use super::begin_java_script_array_transform_element;
use super::begin_java_script_scalar_protocol;
use super::continue_java_script_array_transform_length;
use super::continue_java_script_array_transform_species;
use super::convert_java_script_semantic_boolean;
use super::ml;
use super::ml_string;
use super::read_java_script_array_transform_state_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_array_species_getter;
use super::resume_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_transform_call(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayTransformState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayTransformTransition {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
            shared_constructor_field20,
            _,
            _,
            _,
            shared_constructor_field21,
            _,
            shared_constructor_field22,
            _,
            _,
            shared_constructor_field23,
            _,
            _,
            shared_constructor_field24,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field21 = *shared_constructor_field21;
            let shared_constructor_field23 = *shared_constructor_field23;
            {
                let error = self::read_java_script_array_transform_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if match (
                        match state.clone() {
                            self::JavaScriptArrayTransformState::ArrayTransformState(
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
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value5968,
                            ) => ((source_ml_source2_data_property_value5968
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114,
                                109, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 109, 112, 116,
                                105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99,
                                116, 101, 100, 32, 99, 104, 105, 108, 100,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value5969) => {
                                    let source_ml_source2_data_property_value5969 = *source_ml_source2_data_property_value5969;
                                    source_ml_source2_data_property_value5969.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value5970) => {
                                    let source_ml_source2_data_property_value5970 = *source_ml_source2_data_property_value5970;
                                    source_ml_source2_data_property_value5970.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                    Box::new(heap.clone()),
                                    invalid.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        {
                                            let prior_call = match state.clone() {
                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value5971) => {
                                                    ((source_ml_source2_data_property_value5971.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                }
                                            };
                                            {
                                                let base = match state.clone() {
                                                    self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_rest_field5972, source_ml_source2_nominal_rest_field5973, source_ml_source2_nominal_rest_field5974, source_ml_source2_nominal_rest_field5975, source_ml_source2_nominal_rest_field5976, source_ml_source2_nominal_rest_field5977, source_ml_source2_nominal_rest_field5978, source_ml_source2_nominal_rest_field5979, source_ml_source2_nominal_rest_field5980, source_ml_source2_nominal_rest_field5981, source_ml_source2_nominal_rest_field5982, source_ml_source2_nominal_rest_field5983, source_ml_source2_nominal_rest_field5984, source_ml_source2_nominal_rest_field5985, source_ml_source2_nominal_rest_field5986, source_ml_source2_nominal_rest_field5987, source_ml_source2_nominal_rest_field5988, source_ml_source2_nominal_rest_field5989, _) => {
                                                        let source_ml_source2_nominal_rest_field5973 = *source_ml_source2_nominal_rest_field5973;
                                                        let source_ml_source2_nominal_rest_field5974 = *source_ml_source2_nominal_rest_field5974;
                                                        let source_ml_source2_nominal_rest_field5975 = *source_ml_source2_nominal_rest_field5975;
                                                        let source_ml_source2_nominal_rest_field5976 = *source_ml_source2_nominal_rest_field5976;
                                                        let source_ml_source2_nominal_rest_field5980 = *source_ml_source2_nominal_rest_field5980;
                                                        let source_ml_source2_nominal_rest_field5981 = *source_ml_source2_nominal_rest_field5981;
                                                        let source_ml_source2_nominal_rest_field5982 = *source_ml_source2_nominal_rest_field5982;
                                                        let source_ml_source2_nominal_rest_field5983 = *source_ml_source2_nominal_rest_field5983;
                                                        self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_rest_field5972.clone(), Box::new(source_ml_source2_nominal_rest_field5973.clone()), Box::new(source_ml_source2_nominal_rest_field5974.clone()), Box::new(source_ml_source2_nominal_rest_field5975.clone()), Box::new(source_ml_source2_nominal_rest_field5976.clone()), source_ml_source2_nominal_rest_field5977, source_ml_source2_nominal_rest_field5978, source_ml_source2_nominal_rest_field5979, Box::new(source_ml_source2_nominal_rest_field5980.clone()), Box::new(source_ml_source2_nominal_rest_field5981.clone()), Box::new(source_ml_source2_nominal_rest_field5982.clone()), Box::new(source_ml_source2_nominal_rest_field5983.clone()), source_ml_source2_nominal_rest_field5984.clone(), source_ml_source2_nominal_rest_field5985.clone(), source_ml_source2_nominal_rest_field5986.clone(), source_ml_source2_nominal_rest_field5987.clone(), source_ml_source2_nominal_rest_field5988.clone(), source_ml_source2_nominal_rest_field5989.clone(), None::<(Option<self::JavaScriptArrayTransformCall>,)>)
                                                    }
                                                };
                                                match base.clone() {
                                                    self::JavaScriptArrayTransformState::ArrayTransformState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19) => {
                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                        let shared_constructor_field3 = *shared_constructor_field3;
                                                        let shared_constructor_field4 = *shared_constructor_field4;
                                                        let shared_constructor_field5 = *shared_constructor_field5;
                                                        let shared_constructor_field9 = *shared_constructor_field9;
                                                        let shared_constructor_field10 = *shared_constructor_field10;
                                                        let shared_constructor_field11 = *shared_constructor_field11;
                                                        let shared_constructor_field12 = *shared_constructor_field12;
                                                        if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]))) {
                                                            self::continue_java_script_array_transform_length(heap.clone(), directory.clone(), {
                                                                let source_ml_source2_nominal_field5993 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field5995 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field5997 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5999 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6001 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6003 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6005 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6007 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6009 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6011 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6013 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6015 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6017 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6019 = shared_constructor_field14.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field6021 = shared_constructor_field15.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field6023 = shared_constructor_field16.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field6025 = shared_constructor_field17.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field6027 = shared_constructor_field18.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field6029 = shared_constructor_field19.clone();
                                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5993.clone(), Box::new(source_ml_source2_nominal_field5995.clone()), Box::new(source_ml_source2_nominal_field5997.clone()), Box::new(source_ml_source2_nominal_field5999.clone()), Box::new(source_ml_source2_nominal_field6001.clone()), source_ml_source2_nominal_field6003, source_ml_source2_nominal_field6005, source_ml_source2_nominal_field6007, Box::new(source_ml_source2_nominal_field6009.clone()), Box::new(source_ml_source2_nominal_field6011.clone()), Box::new(source_ml_source2_nominal_field6013.clone()), Box::new(value.clone()), source_ml_source2_nominal_field6017.clone(), source_ml_source2_nominal_field6019.clone(), source_ml_source2_nominal_field6021.clone(), source_ml_source2_nominal_field6023.clone(), source_ml_source2_nominal_field6025.clone(), source_ml_source2_nominal_field6027.clone(), source_ml_source2_nominal_field6029.clone())
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
                                                            }, self::begin_java_script_scalar_protocol(heap.clone(), value.clone(), shared_constructor_field21.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                        } else {
                                                            if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                                if match (match state.clone() {
                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value6035, _, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value6035.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                } else {
                                                                    self::continue_java_script_array_transform_length(heap.clone(), directory.clone(), base.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value6036, _, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value6036.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"), completion.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                }
                                                            } else {
                                                                if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) {
                                                                    if match (match state.clone() {
                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value6039, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value6039.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110]))
                                                                    } else {
                                                                        self::continue_java_script_array_transform_species(heap.clone(), base.clone(), self::resume_java_script_array_species_getter(heap.clone(), directory.clone(), (match state.clone() {
                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value6040, _, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value6040.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"), completion.clone()))
                                                                    }
                                                                } else {
                                                                    if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                        self::begin_java_script_array_transform_element(heap.clone(), base.clone(), value.clone())
                                                                    } else {
                                                                        if (shared_constructor_field24 != (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]))) {
                                                                            self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101]))
                                                                        } else {
                                                                            if ((shared_constructor_field20 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 108, 116, 101, 114]))) && !(self::convert_java_script_semantic_boolean(value.clone()))) {
                                                                                self::JavaScriptArrayTransformTransition::ArrayTransformContinue(Box::new(heap.clone()), Box::new({
                                                                                    let source_ml_source2_nominal_field6049 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6051 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6053 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6055 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6057 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6059 = shared_constructor_field6;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6061 = shared_constructor_field7;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6063 = shared_constructor_field8;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6065 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field6067 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field6069 = shared_constructor_field11.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field6071 = shared_constructor_field12.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field6073 = shared_constructor_field13.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field6075 = shared_constructor_field14.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field6077 = shared_constructor_field15.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field6079 = shared_constructor_field16.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field6081 = shared_constructor_field17.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field6083 = shared_constructor_field18.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field6085 = shared_constructor_field19.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override6088 = (shared_constructor_field22 + 1f64);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_override6089 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field6049.clone(), Box::new(source_ml_source2_nominal_field6051.clone()), Box::new(source_ml_source2_nominal_field6053.clone()), Box::new(source_ml_source2_nominal_field6055.clone()), Box::new(source_ml_source2_nominal_field6057.clone()), source_ml_source2_nominal_field6059, source_ml_source2_nominal_override6088, source_ml_source2_nominal_field6063, Box::new(source_ml_source2_nominal_field6065.clone()), Box::new(source_ml_source2_nominal_override6089.clone()), Box::new(source_ml_source2_nominal_field6069.clone()), Box::new(source_ml_source2_nominal_field6071.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field6075.clone(), source_ml_source2_nominal_field6077.clone(), source_ml_source2_nominal_field6079.clone(), source_ml_source2_nominal_field6081.clone(), source_ml_source2_nominal_field6083.clone(), source_ml_source2_nominal_field6085.clone())
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
                                                                                }))
                                                                            } else {
                                                                                self::JavaScriptArrayTransformTransition::ArrayTransformContinue(Box::new(heap.clone()), Box::new({
                                                                                    let source_ml_source2_nominal_field6094 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6096 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6098 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6100 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6102 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6104 = shared_constructor_field6;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6106 = shared_constructor_field7;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6108 = shared_constructor_field8;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6110 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field6112 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field6114 = shared_constructor_field11.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field6116 = shared_constructor_field12.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field6118 = shared_constructor_field13.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field6120 = shared_constructor_field14.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field6122 = shared_constructor_field15.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field6124 = shared_constructor_field16.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field6126 = shared_constructor_field17.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field6128 = shared_constructor_field18.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field6130 = shared_constructor_field19.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override6133 = if (shared_constructor_field20 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                                                                                                                                                    value.clone()
                                                                                                                                                                } else {
                                                                                                                                                                    shared_constructor_field23.clone()
                                                                                                                                                                };
                                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field6094.clone(), Box::new(source_ml_source2_nominal_field6096.clone()), Box::new(source_ml_source2_nominal_field6098.clone()), Box::new(source_ml_source2_nominal_field6100.clone()), Box::new(source_ml_source2_nominal_field6102.clone()), source_ml_source2_nominal_field6104, source_ml_source2_nominal_field6106, source_ml_source2_nominal_field6108, Box::new(source_ml_source2_nominal_field6110.clone()), Box::new(source_ml_source2_nominal_field6112.clone()), Box::new(source_ml_source2_nominal_override6133.clone()), Box::new(source_ml_source2_nominal_field6116.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field6120.clone(), source_ml_source2_nominal_field6122.clone(), source_ml_source2_nominal_field6124.clone(), source_ml_source2_nominal_field6126.clone(), source_ml_source2_nominal_field6128.clone(), source_ml_source2_nominal_field6130.clone())
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
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArrayTransformTransition::ArrayTransformThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
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
pub fn resume_java_script_array_transform_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayTransformState,
    identity: self::ml::Big,
) -> self::JavaScriptArrayTransformTransition {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
            shared_constructor_field1,
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
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_array_transform_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if ((shared_constructor_field13
                        != (self::ml_string::Text::from_units(vec![
                            97, 108, 108, 111, 99, 97, 116, 101,
                        ])))
                        || match (
                            match state.clone() {
                                self::JavaScriptArrayTransformState::ArrayTransformState(
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
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6136,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value6136
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    {
                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                111, 110, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32,
                                114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 99, 114,
                                101, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 101, 115, 116,
                            ]),
                        )
                    } else {
                        {
                            let created = self::allocate_java_script_array(
                                heap.clone(),
                                identity.clone(),
                                (match state.clone() {
                                    self::JavaScriptArrayTransformState::ArrayTransformState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value6137,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value6137
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"),
                                if (shared_constructor_field1
                                    == (self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 46, 109, 97, 112,
                                    ])))
                                {
                                    shared_constructor_field6
                                } else {
                                    0f64
                                },
                            );
                            match created.clone() {
                                self::JavaScriptArrayCompletion::ArrayNormal(heap_2, value) => {
                                    let heap_2 = *heap_2;
                                    let value = *value;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformContinue(
                                        Box::new(heap_2.clone()),
                                        Box::new({
                                            let source_ml_source2_nominal_field6149 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field6151 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field6153 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field6155 =
                                                            shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field6157 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field6159 =
                                                                    shared_constructor_field6;
                                                                {
                                                                    let source_ml_source2_nominal_field6161 =
                                                                        shared_constructor_field7;
                                                                    {
                                                                        let source_ml_source2_nominal_field6163 = shared_constructor_field8;
                                                                        {
                                                                            let source_ml_source2_nominal_field6165 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6167 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6169 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6171 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6173 = shared_constructor_field13.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6175 = shared_constructor_field14.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6177 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6179 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6181 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6183 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6185 = shared_constructor_field19.clone();
                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field6149.clone(), Box::new(source_ml_source2_nominal_field6151.clone()), Box::new(source_ml_source2_nominal_field6153.clone()), Box::new(source_ml_source2_nominal_field6155.clone()), Box::new(source_ml_source2_nominal_field6157.clone()), source_ml_source2_nominal_field6159, source_ml_source2_nominal_field6161, source_ml_source2_nominal_field6163, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field6167.clone()), Box::new(source_ml_source2_nominal_field6169.clone()), Box::new(source_ml_source2_nominal_field6171.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field6175.clone(), source_ml_source2_nominal_field6177.clone(), source_ml_source2_nominal_field6179.clone(), source_ml_source2_nominal_field6181.clone(), source_ml_source2_nominal_field6183.clone(), source_ml_source2_nominal_field6185.clone())
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
                                        }),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayPropertyComplete(
                                    heap_3,
                                    _,
                                    _,
                                ) => {
                                    let heap_3 = *heap_3;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                        Box::new(heap_3.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105,
                                            111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117,
                                            98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 110, 32,
                                            117, 110, 114, 101, 108, 97, 116, 101, 100, 32, 100,
                                            101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99,
                                            111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110,
                                        ]),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayNumberRequest(
                                    heap_4,
                                    _,
                                    _,
                                ) => {
                                    let heap_4 = *heap_4;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                        Box::new(heap_4.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105,
                                            111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117,
                                            98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 110, 32,
                                            117, 110, 114, 101, 108, 97, 116, 101, 100, 32, 100,
                                            101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99,
                                            111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110,
                                        ]),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayDefinitionContinue(
                                    heap_5,
                                    _,
                                ) => {
                                    let heap_5 = *heap_5;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                        Box::new(heap_5.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105,
                                            111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117,
                                            98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 110, 32,
                                            117, 110, 114, 101, 108, 97, 116, 101, 100, 32, 100,
                                            101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99,
                                            111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110,
                                        ]),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayInvalid(
                                    heap_6,
                                    source_ml_source2_f6146,
                                ) => {
                                    let heap_6 = *heap_6;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                        Box::new(heap_6.clone()),
                                        source_ml_source2_f6146.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayError(
                                    heap_7,
                                    error_class,
                                    reason,
                                ) => {
                                    let heap_7 = *heap_7;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformError(
                                        Box::new(heap_7.clone()),
                                        error_class.clone(),
                                        reason.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                    heap_8,
                                    reason_2,
                                    _,
                                ) => {
                                    let heap_8 = *heap_8;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(Box::new(heap_8.clone()), Box::new(state.clone()), reason_2.clone())
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
pub fn resume_java_script_array_transform_construction(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayTransformState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayTransformTransition {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
            shared_constructor_field1,
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
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_array_transform_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (shared_constructor_field13
                        != (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116,
                        ])))
                    {
                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 117, 115, 116, 111, 109, 32, 99, 111, 110, 115, 116, 114, 117,
                                99, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105,
                                111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116,
                                101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value6195) => {
                                    let source_ml_source2_data_property_value6195 = *source_ml_source2_data_property_value6195;
                                    source_ml_source2_data_property_value6195.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value6196) => {
                                    let source_ml_source2_data_property_value6196 = *source_ml_source2_data_property_value6196;
                                    source_ml_source2_data_property_value6196.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                    Box::new(heap.clone()),
                                    invalid.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if match value.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } {
                                            self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 115, 117, 108, 116, 32, 79, 98, 106, 101, 99, 116]))
                                        } else {
                                            self::JavaScriptArrayTransformTransition::ArrayTransformContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field6200 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field6202 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field6204 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field6206 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field6208 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field6210 = shared_constructor_field6;
                                                                    {
                                                                        let source_ml_source2_nominal_field6212 = shared_constructor_field7;
                                                                        {
                                                                            let source_ml_source2_nominal_field6214 = shared_constructor_field8;
                                                                            {
                                                                                let source_ml_source2_nominal_field6216 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6218 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6220 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6222 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6224 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6226 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6228 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6230 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6232 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6234 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field6236 = shared_constructor_field19.clone();
                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field6200.clone(), Box::new(source_ml_source2_nominal_field6202.clone()), Box::new(source_ml_source2_nominal_field6204.clone()), Box::new(source_ml_source2_nominal_field6206.clone()), Box::new(source_ml_source2_nominal_field6208.clone()), source_ml_source2_nominal_field6210, source_ml_source2_nominal_field6212, source_ml_source2_nominal_field6214, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field6218.clone()), Box::new(source_ml_source2_nominal_field6220.clone()), Box::new(source_ml_source2_nominal_field6222.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field6226.clone(), source_ml_source2_nominal_field6228.clone(), source_ml_source2_nominal_field6230.clone(), source_ml_source2_nominal_field6232.clone(), source_ml_source2_nominal_field6234.clone(), source_ml_source2_nominal_field6236.clone())
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
                                            }))
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArrayTransformTransition::ArrayTransformThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
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
