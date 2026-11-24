// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionsCall;
use super::JavaScriptPropertyDefinitionsState;
use super::JavaScriptPropertyDefinitionsTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_definition_state_error;
use super::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1031;
use super::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1032;
use super::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1033;
use super::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1037;
use super::test_java_script_property_definitions_operation;
use super::test_java_script_property_descriptors_same_value;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_definitions_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionsState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            {
                let invalid = self::read_java_script_ordinary_heap_error(heap.clone());
                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                    invalid.clone()
                } else {
                    if (((((((((((!(self::test_java_script_property_definitions_operation(
                        &(shared_constructor_field1),
                    ))
                        || !(self::ml_array::includes(
                            &{
                                let ml_reference_value1103 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            97, 108, 108, 111, 99, 97, 116, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                                        self::ml_string::Text::from_units(vec![114, 101, 97, 100]),
                                        self::ml_string::Text::from_units(vec![
                                            103, 101, 116, 116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 118, 101, 114, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 105, 110, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 105, 110, 105, 116, 105, 111, 110,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value1103
                            },
                            &shared_constructor_field6.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )))
                        || (({
                            let ml_reference_array1104 = shared_constructor_field7.clone();
                            let ml_reference_projection1104 =
                                ml_reference_array1104.borrow().len() as f64;
                            ml_reference_projection1104
                        }) > (4096f64)))
                        || (({
                            let ml_reference_array1105 = shared_constructor_field9.clone();
                            let ml_reference_projection1105 =
                                ml_reference_array1105.borrow().len() as f64;
                            ml_reference_projection1105
                        }) > shared_constructor_field8))
                        || !(self::ml_math::is_safe_integer(shared_constructor_field8)))
                        || {
                            let source_ml_source2_same_value_operand21779 =
                                shared_constructor_field8;
                            {
                                let source_ml_source2_same_value_operand21780 = (-0f64);
                                (((source_ml_source2_same_value_operand21779
                                    == source_ml_source2_same_value_operand21780)
                                    && ((source_ml_source2_same_value_operand21779 != (0f64))
                                        || ((1f64 / source_ml_source2_same_value_operand21779)
                                            == (1f64
                                                / source_ml_source2_same_value_operand21780))))
                                    || ((source_ml_source2_same_value_operand21779
                                        != source_ml_source2_same_value_operand21779)
                                        && (source_ml_source2_same_value_operand21780
                                            != source_ml_source2_same_value_operand21780)))
                            }
                        })
                        || (shared_constructor_field8 < (0f64)))
                        || (shared_constructor_field8
                            > ({
                                let ml_reference_array1106 = shared_constructor_field7.clone();
                                let ml_reference_projection1106 =
                                    ml_reference_array1106.borrow().len() as f64;
                                ml_reference_projection1106
                            })))
                        || !(self::ml_math::is_safe_integer(shared_constructor_field10)))
                        || {
                            let source_ml_source2_same_value_operand21789 =
                                shared_constructor_field10;
                            {
                                let source_ml_source2_same_value_operand21790 = (-0f64);
                                (((source_ml_source2_same_value_operand21789
                                    == source_ml_source2_same_value_operand21790)
                                    && ((source_ml_source2_same_value_operand21789 != (0f64))
                                        || ((1f64 / source_ml_source2_same_value_operand21789)
                                            == (1f64
                                                / source_ml_source2_same_value_operand21790))))
                                    || ((source_ml_source2_same_value_operand21789
                                        != source_ml_source2_same_value_operand21789)
                                        && (source_ml_source2_same_value_operand21790
                                            != source_ml_source2_same_value_operand21790)))
                            }
                        })
                        || (shared_constructor_field10 < (0f64)))
                        || (shared_constructor_field10
                            > ({
                                let ml_reference_array1107 = shared_constructor_field9.clone();
                                let ml_reference_projection1107 =
                                    ml_reference_array1107.borrow().len() as f64;
                                ml_reference_projection1107
                            })))
                    {
                        self::ml_string::Text::from_units(vec![
                            98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111,
                            110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114,
                            32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105,
                            111, 110, 32, 112, 104, 97, 115, 101, 32, 97, 110, 100, 32, 98, 111,
                            117, 110, 100, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32,
                            100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 112, 111, 115,
                            105, 116, 105, 111, 110, 115,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values1026 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                    shared_constructor_field5.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index1026 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s21801 = self::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1031(heap.clone(), source_ml_source1_temporary_values1026.clone(), source_ml_source1_temporary_index1026.clone());
                                    match source_ml_source2_s21801.clone() {
                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Done => {
                                            if (({
                                                let source_ml_source2_s21803 = shared_constructor_field5.clone();
                                                match source_ml_source2_s21803.clone() {
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            } || ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) && (((shared_constructor_field1 != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101]))) || {
                                                let source_ml_source2_s21811 = shared_constructor_field2.clone();
                                                match source_ml_source2_s21811.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) || ({
                                                let source_ml_source2_s21814 = shared_constructor_field4.clone();
                                                match source_ml_source2_s21814.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            } && {
                                                let source_ml_source2_s21819 = shared_constructor_field4.clone();
                                                match source_ml_source2_s21819.clone() {
                                                    self::JavaScriptSemanticValue::Null => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            })))) || ((shared_constructor_field6 != (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) && {
                                                let source_ml_source2_s21824 = shared_constructor_field2.clone();
                                                match source_ml_source2_s21824.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            })) {
                                                self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                            } else {
                                                {
                                                    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let source_ml_source1_temporary_values1027 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source1_temporary_index1027 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s21829 = self::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1032(heap.clone(), seen.clone(), source_ml_source1_temporary_values1027.clone(), source_ml_source1_temporary_index1027.clone());
                                                                match source_ml_source2_s21829.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Done => {
                                                                        {
                                                                            let source_ml_source1_temporary_values1028 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source1_temporary_index1028 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source2_s21831 = self::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1033(heap.clone(), source_ml_source1_temporary_values1028.clone(), source_ml_source1_temporary_index1028.clone());
                                                                                    match source_ml_source2_s21831.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Done => {
                                                                                            if ((((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])))) != (!(match (match state.clone() {
                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21837, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value21837.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                                                                                }
                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) || (((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110])))) != (!(match (match state.clone() {
                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21843, _) => {
                                                                                                    ((source_ml_source2_data_property_value21843.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                }
                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })))) {
                                                                                                self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source2_s21852 = if !(match (match state.clone() {
                                                                                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21853, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value21853.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                        {
                                                                                                            let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), (match state.clone() {
                                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21854, _, _) => {
                                                                                                                    ((source_ml_source2_data_property_value21854.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value"));
                                                                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Return(error.clone())
                                                                                                            } else {
                                                                                                                if ((match (match state.clone() {
                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21858, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value21858.clone().unwrap_or_else(|| (None::<self::JavaScriptDescriptorConversionState>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value21859) => {
                                                                                                                        source_ml_source2_data_property_value21859
                                                                                                                    }
                                                                                                                }) != (!(match (match state.clone() {
                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21860) => {
                                                                                                                        ((source_ml_source2_data_property_value21860.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                    }
                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 71, 101, 116, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Next
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Next
                                                                                                    };
                                                                                                    match source_ml_source2_s21852.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Next => {
                                                                                                            {
                                                                                                                let source_ml_source2_s21932 = if !(match (match state.clone() {
                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21933, _) => {
                                                                                                                        ((source_ml_source2_data_property_value21933.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                    }
                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                    {
                                                                                                                        let error_2 = self::read_java_script_property_definition_state_error(heap.clone(), (match state.clone() {
                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21934, _) => {
                                                                                                                                ((source_ml_source2_data_property_value21934.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                            }
                                                                                                                        }).clone().expect("a proven present value"));
                                                                                                                        if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(error_2.clone())
                                                                                                                        } else {
                                                                                                                            if (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21935, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value21935.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                }
                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value21936, _, _, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value21936 = *source_ml_source2_data_property_value21936;
                                                                                                                                    source_ml_source2_data_property_value21936.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value21937, _, _, _, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value21937 = *source_ml_source2_data_property_value21937;
                                                                                                                                    source_ml_source2_data_property_value21937.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(source_ml_source2_data_property_value21938, _, _, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value21938 = *source_ml_source2_data_property_value21938;
                                                                                                                                    source_ml_source2_data_property_value21938.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(source_ml_source2_data_property_value21939, _, _, _, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value21939 = *source_ml_source2_data_property_value21939;
                                                                                                                                    source_ml_source2_data_property_value21939.clone()
                                                                                                                                }
                                                                                                                            }, shared_constructor_field2.clone())) || {
                                                                                                                                let source_ml_source2_comparison_operand21946 = match (match state.clone() {
                                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21941, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value21941.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, source_ml_source2_data_property_value21942, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value21942.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                    }
                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, source_ml_source2_data_property_value21943, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value21943.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                    }
                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, source_ml_source2_data_property_value21944, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value21944.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                    }
                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, source_ml_source2_data_property_value21945, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value21945.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                match (source_ml_source2_comparison_operand21946.clone(), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                                                            }) {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 116, 97, 114, 103, 101, 116, 32, 97, 110, 100, 32, 116, 104, 114, 111, 119, 105, 110, 103, 32, 114, 101, 115, 117, 108, 116, 32, 112, 111, 108, 105, 99, 121]))
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let selected = {
                                                                                                                                        let source_ml_source2_array_read_value21950 = shared_constructor_field9.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_array_read_index21951 = shared_constructor_field10;
                                                                                                                                            { let ml_reference_array1108 = source_ml_source2_array_read_value21950.clone(); let ml_reference_position1108 = source_ml_source2_array_read_index21951; let ml_reference_projection1108 = self::ml_array::optional_at(&*ml_reference_array1108.borrow(), self::ml_array::number_index(ml_reference_position1108)); ml_reference_projection1108 }
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    if ((match (selected.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ({
                                                                                                                                        let source_ml_source2_s21953 = (match state.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21959, _) => {
                                                                                                                                                ((source_ml_source2_data_property_value21959.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value");
                                                                                                                                        match source_ml_source2_s21953.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, _) => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    } && {
                                                                                                                                        let source_ml_source2_s21961 = (match state.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21968, _) => {
                                                                                                                                                ((source_ml_source2_data_property_value21968.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value");
                                                                                                                                        match source_ml_source2_s21961.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, _) => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    })) || !(self::test_java_script_semantic_same_value((((match (match state.clone() {
                                                                                                                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21969, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value21969.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, source_ml_source2_data_property_value21970, _, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value21970 = *source_ml_source2_data_property_value21970;
                                                                                                                                            Some((source_ml_source2_data_property_value21970.clone(),))
                                                                                                                                        }
                                                                                                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, source_ml_source2_data_property_value21971, _, _, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value21971 = *source_ml_source2_data_property_value21971;
                                                                                                                                            Some((source_ml_source2_data_property_value21971.clone(),))
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            None::<(self::JavaScriptSemanticValue,)>
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value")).0).clone(), match selected.clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value21972, _) => {
                                                                                                                                            let source_ml_source2_data_property_value21972 = *source_ml_source2_data_property_value21972;
                                                                                                                                            source_ml_source2_data_property_value21972.clone()
                                                                                                                                        }
                                                                                                                                    }))) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let descriptor = if {
                                                                                                                                                let source_ml_source2_s21974 = (match state.clone() {
                                                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21980, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value21980.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value");
                                                                                                                                                match source_ml_source2_s21974.clone() {
                                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, _) => {
                                                                                                                                                        true
                                                                                                                                                    }
                                                                                                                                                    _ => {
                                                                                                                                                        false
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                match match (((match (match state.clone() {
                                                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21981, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value21981.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, source_ml_source2_data_property_value21982, _, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value21982 = *source_ml_source2_data_property_value21982;
                                                                                                                                                        Some((source_ml_source2_data_property_value21982.clone(),))
                                                                                                                                                    }
                                                                                                                                                    _ => {
                                                                                                                                                        None::<(self::JavaScriptArrayAssignmentState,)>
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value")).0).clone() {
                                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value21983, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value21983 = *source_ml_source2_data_property_value21983;
                                                                                                                                                        source_ml_source2_data_property_value21983.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value21984, _, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value21984 = *source_ml_source2_data_property_value21984;
                                                                                                                                                        source_ml_source2_data_property_value21984.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value21985, _, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value21985 = *source_ml_source2_data_property_value21985;
                                                                                                                                                        source_ml_source2_data_property_value21985.clone()
                                                                                                                                                    }
                                                                                                                                                } {
                                                                                                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value21986, _, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21986.clone()
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                (((match (match state.clone() {
                                                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21987, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value21987.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, source_ml_source2_data_property_value21988, _, _, _) => {
                                                                                                                                                        Some((source_ml_source2_data_property_value21988.clone(),))
                                                                                                                                                    }
                                                                                                                                                    _ => {
                                                                                                                                                        None::<(std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>,)>
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value")).0).clone()
                                                                                                                                            };
                                                                                                                                            if !(self::test_java_script_property_descriptors_same_value(descriptor.clone(), match selected.clone().expect("a proven present value") {
                                                                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value21989) => {
                                                                                                                                                    source_ml_source2_data_property_value21989.clone()
                                                                                                                                                }
                                                                                                                                            })) {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 101, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]))
                                                                                                                                            } else {
                                                                                                                                                if ({
                                                                                                                                                    let source_ml_source2_s21991 = (match state.clone() {
                                                                                                                                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21997, _) => {
                                                                                                                                                            ((source_ml_source2_data_property_value21997.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                                        }
                                                                                                                                                    }).clone().expect("a proven present value");
                                                                                                                                                    match source_ml_source2_s21991.clone() {
                                                                                                                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, _) => {
                                                                                                                                                            true
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            false
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                } && !(self::test_java_script_semantic_same_value(match (((match (match state.clone() {
                                                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21998, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value21998.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionState>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, source_ml_source2_data_property_value21999, _, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value21999 = *source_ml_source2_data_property_value21999;
                                                                                                                                                        Some((source_ml_source2_data_property_value21999.clone(),))
                                                                                                                                                    }
                                                                                                                                                    _ => {
                                                                                                                                                        None::<(self::JavaScriptArrayAssignmentState,)>
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value")).0).clone() {
                                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(_, source_ml_source2_data_property_value22000) => {
                                                                                                                                                        let source_ml_source2_data_property_value22000 = *source_ml_source2_data_property_value22000;
                                                                                                                                                        source_ml_source2_data_property_value22000.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, source_ml_source2_data_property_value22001, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value22001 = *source_ml_source2_data_property_value22001;
                                                                                                                                                        source_ml_source2_data_property_value22001.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(_, source_ml_source2_data_property_value22002, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value22002 = *source_ml_source2_data_property_value22002;
                                                                                                                                                        source_ml_source2_data_property_value22002.clone()
                                                                                                                                                    }
                                                                                                                                                }, shared_constructor_field5.clone()))) {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108]))
                                                                                                                                                } else {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Next
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Next
                                                                                                                };
                                                                                                                match source_ml_source2_s21932.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Next => {
                                                                                                                        if (((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) && match (match state.clone() {
                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22006) => {
                                                                                                                                ((source_ml_source2_data_property_value22006.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                            }
                                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || (!(match (match state.clone() {
                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22007) => {
                                                                                                                                ((source_ml_source2_data_property_value22007.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                            }
                                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::ml_array::includes(&{ let ml_reference_value1109 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value1109 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b)))) {
                                                                                                                            self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108, 115])
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let source_ml_source2_s22027 = if !(match (match state.clone() {
                                                                                                                                    self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22028) => {
                                                                                                                                        ((source_ml_source2_data_property_value22028.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_temporary_values1030 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22029) => {
                                                                                                                                                ((source_ml_source2_data_property_value22029.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(source_ml_source2_data_property_value22030, _, _) => {
                                                                                                                                                let source_ml_source2_data_property_value22030 = *source_ml_source2_data_property_value22030;
                                                                                                                                                source_ml_source2_data_property_value22030.clone()
                                                                                                                                            }
                                                                                                                                        }, match (match state.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22031) => {
                                                                                                                                                ((source_ml_source2_data_property_value22031.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(_, source_ml_source2_data_property_value22032, _) => {
                                                                                                                                                let source_ml_source2_data_property_value22032 = *source_ml_source2_data_property_value22032;
                                                                                                                                                source_ml_source2_data_property_value22032.clone()
                                                                                                                                            }
                                                                                                                                        }], { let ml_reference_value1110 = (match (match state.clone() {
                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22033) => {
                                                                                                                                                ((source_ml_source2_data_property_value22033.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(_, _, source_ml_source2_data_property_value22034) => {
                                                                                                                                                source_ml_source2_data_property_value22034.clone()
                                                                                                                                            }
                                                                                                                                        }).borrow().clone(); ml_reference_value1110 })));
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_temporary_index1030 = self::ml::Big::from_i128(0);
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_s22035 = self::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1037(heap.clone(), source_ml_source1_temporary_values1030.clone(), source_ml_source1_temporary_index1030.clone());
                                                                                                                                                match source_ml_source2_s22035.clone() {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Done => {
                                                                                                                                                        if ({
                                                                                                                                                            let source_ml_source2_s22038 = match (match state.clone() {
                                                                                                                                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22041) => {
                                                                                                                                                                    ((source_ml_source2_data_property_value22041.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                                                                }
                                                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                                                self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(source_ml_source2_data_property_value22042, _, _) => {
                                                                                                                                                                    let source_ml_source2_data_property_value22042 = *source_ml_source2_data_property_value22042;
                                                                                                                                                                    source_ml_source2_data_property_value22042.clone()
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            match source_ml_source2_s22038.clone() {
                                                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                                                    false
                                                                                                                                                                }
                                                                                                                                                                _ => {
                                                                                                                                                                    true
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } || !(((match match (match state.clone() {
                                                                                                                                                            self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22043) => {
                                                                                                                                                                ((source_ml_source2_data_property_value22043.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                                                                                                                            }
                                                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                                                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(source_ml_source2_data_property_value22044, _, _) => {
                                                                                                                                                                let source_ml_source2_data_property_value22044 = *source_ml_source2_data_property_value22044;
                                                                                                                                                                source_ml_source2_data_property_value22044.clone()
                                                                                                                                                            }
                                                                                                                                                        } {
                                                                                                                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value22045) => {
                                                                                                                                                                Some((source_ml_source2_data_property_value22045,))
                                                                                                                                                            }
                                                                                                                                                            _ => {
                                                                                                                                                                None::<(bool,)>
                                                                                                                                                            }
                                                                                                                                                        }).clone().expect("a proven present value")).0)) {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121]))
                                                                                                                                                        } else {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Next
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Return(source_ml_source1_temporary_result1037) => {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Return(source_ml_source1_temporary_result1037.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Next
                                                                                                                                };
                                                                                                                                match source_ml_source2_s22027.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Next => {
                                                                                                                                        if (((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]))) || (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110])))) && (shared_constructor_field8 != ({ let ml_reference_array1111 = shared_constructor_field7.clone(); let ml_reference_projection1111 = ml_reference_array1111.borrow().len() as f64; ml_reference_projection1111 }))) {
                                                                                                                                            self::ml_string::Text::from_units(vec![97, 108, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115, 32, 99, 111, 110, 118, 101, 114, 116, 32, 98, 101, 102, 111, 114, 101, 32, 116, 104, 101, 32, 102, 105, 114, 115, 116, 32, 98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                                                                                                                        } else {
                                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Return(source_ml_source1_temporary_result1036) => {
                                                                                                                                        source_ml_source1_temporary_result1036.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(source_ml_source1_temporary_result1037_2) => {
                                                                                                                        source_ml_source1_temporary_result1037_2.clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Return(source_ml_source1_temporary_result1037_3) => {
                                                                                                            source_ml_source1_temporary_result1037_3.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Return(source_ml_source1_temporary_result1037_4) => {
                                                                                            source_ml_source1_temporary_result1037_4.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Return(source_ml_source1_temporary_result1038) => {
                                                                        source_ml_source1_temporary_result1038.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Return(source_ml_source1_temporary_result1039) => {
                                            source_ml_source1_temporary_result1039.clone()
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
pub fn begin_java_script_property_definitions(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    target: self::JavaScriptSemanticValue,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptPropertyDefinitionsTransition {
    {
        let error = {
            let source_ml_source2_logical22055 = {
                let source_ml_source2_logical22054 =
                    self::read_java_script_ordinary_heap_error(heap.clone());
                if !source_ml_source2_logical22054.0.is_empty() {
                    source_ml_source2_logical22054.clone()
                } else {
                    self::read_java_script_ordinary_value_error(heap.clone(), target.clone())
                }
            };
            if !source_ml_source2_logical22055.0.is_empty() {
                source_ml_source2_logical22055.clone()
            } else {
                self::read_java_script_ordinary_value_error(heap.clone(), input.clone())
            }
        };
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101,
                ])))
            {
                (match target.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } && match target.clone() {
                    self::JavaScriptSemanticValue::Null => false,
                    _ => true,
                })
            } else {
                match target.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                }
            } {
                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101,
                        ])))
                    {
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112,
                            101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106,
                            101, 99, 116, 32, 111, 114, 32, 110, 117, 108, 108,
                        ])
                    } else {
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114,
                            111, 112, 101, 114, 116, 105, 101, 115, 32, 114, 101, 113, 117, 105,
                            114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97,
                            114, 103, 101, 116,
                        ])
                    },
                )
            } else {
                {
                    let state = {
                        let source_ml_source2_constructor_field22061 = if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101,
                            ]))) {
                            self::JavaScriptSemanticValue::Undefined
                        } else {
                            target.clone()
                        };
                        {
                            let source_ml_source2_constructor_field22063 = if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101,
                                ]))) {
                                target.clone()
                            } else {
                                self::JavaScriptSemanticValue::Undefined
                            };
                            {
                                let source_ml_source2_constructor_field22065 = if ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101,
                                    ]))) {
                                    self::ml_string::Text::from_units(vec![
                                        97, 108, 108, 111, 99, 97, 116, 101,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![107, 101, 121, 115])
                                };
                                {
                                    let source_ml_source2_constructor_field22066 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptSemanticValue,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_source2_constructor_field22067 = 0f64;
                                        {
                                            let source_ml_source2_constructor_field22068 =
                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                    self::JavaScriptOrdinaryProperty,
                                                >::new(
                                                )));
                                            {
                                                let source_ml_source2_constructor_field22069 = 0f64;
                                                self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(operation.clone(), Box::new(source_ml_source2_constructor_field22061.clone()), Box::new(input.clone()), Box::new(source_ml_source2_constructor_field22063.clone()), Box::new(exotic_key.clone()), source_ml_source2_constructor_field22065.clone(), source_ml_source2_constructor_field22066.clone(), source_ml_source2_constructor_field22067, source_ml_source2_constructor_field22068.clone(), source_ml_source2_constructor_field22069, None::<(Option<self::JavaScriptDescriptorConversionState>,)>, None::<(Option<self::JavaScriptPropertyDefinitionState>,)>, None::<(Option<self::JavaScriptPropertyDefinitionsCall>,)>)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                    {
                        let invalid = self::read_java_script_property_definitions_state_error(
                            heap.clone(),
                            state.clone(),
                        );
                        if (invalid == (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(Box::new(heap.clone()), Box::new(state.clone()))
                        } else {
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new(heap.clone()), invalid.clone())
                        }
                    }
                }
            }
        }
    }
}
