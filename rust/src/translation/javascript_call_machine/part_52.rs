// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_concatenation_child_error;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_array_species_state_error;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_array_concatenation_state_error_loop323;
use super::source_ml_source1_temporary_read_java_script_array_concatenation_state_error_loop324;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_concatenation_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayConcatenationState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let directory_error = self::read_java_script_array_species_directory_error(
                    heap.clone(),
                    directory.clone(),
                );
                if (directory_error != (self::ml_string::Text::from_units(vec![]))) {
                    directory_error.clone()
                } else {
                    if ((((match state.clone() {
                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
                            _,
                            _,
                            _,
                            _,
                        ) => false,
                    } || {
                        let source_ml_source2_s8511 = shared_constructor_field1.clone();
                        match source_ml_source2_s8511.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || {
                        let source_ml_source2_s8516 = shared_constructor_field3.clone();
                        match source_ml_source2_s8516.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    }) || {
                        let source_ml_source2_s8520 = shared_constructor_field4.clone();
                        match source_ml_source2_s8520.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    }) || (({
                        let source_ml_source2_type_query_operand8526 = shared_constructor_field11;
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 111, 111, 108, 101, 97, 110,
                    ])))) {
                        self::ml_string::Text::from_units(vec![
                            99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114,
                            101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                            118, 101, 114, 32, 97, 110, 100, 32, 115, 112, 114, 101, 97, 100, 97,
                            98, 105, 108, 105, 116, 121, 47, 99, 111, 110, 118, 101, 114, 115, 105,
                            111, 110, 32, 83, 121, 109, 98, 111, 108, 115,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values320 =
                                std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(
                                    self::ml_array::append(
                                        vec![shared_constructor_field1.clone()],
                                        {
                                            let ml_reference_value680 =
                                                shared_constructor_field2.borrow().clone();
                                            ml_reference_value680
                                        },
                                    ),
                                    vec![
                                        shared_constructor_field3.clone(),
                                        shared_constructor_field4.clone(),
                                        shared_constructor_field6.clone(),
                                        shared_constructor_field12.clone(),
                                        shared_constructor_field13.clone(),
                                        shared_constructor_field14.clone(),
                                    ],
                                )));
                            {
                                let source_ml_source1_temporary_index320 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s8535 = self::source_ml_source1_temporary_read_java_script_array_concatenation_state_error_loop323(heap.clone(), source_ml_source1_temporary_values320.clone(), source_ml_source1_temporary_index320.clone());
                                    match source_ml_source2_s8535.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Done => {
                                            {
                                                let source_ml_source1_temporary_values321 = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field5, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10]));
                                                {
                                                    let source_ml_source1_temporary_index321 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s8541 = self::source_ml_source1_temporary_read_java_script_array_concatenation_state_error_loop324(source_ml_source1_temporary_values321.clone(), source_ml_source1_temporary_index321.clone());
                                                        match source_ml_source2_s8541.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Done => {
                                                                if (((shared_constructor_field5 > (({ let ml_reference_array681 = shared_constructor_field2.clone(); let ml_reference_projection681 = ml_reference_array681.borrow().len() as f64; ml_reference_projection681 } + 1f64))) || (shared_constructor_field9 > shared_constructor_field8)) || (shared_constructor_field7 > shared_constructor_field10)) {
                                                                    self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 109, 32, 76, 105, 115, 116, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 115, 112, 114, 101, 97, 100, 32, 105, 110, 116, 101, 114, 118, 97, 108])
                                                                } else {
                                                                    if !(self::ml_array::includes(&{ let ml_reference_value682 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101]), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121]), self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])]))).borrow().clone(); ml_reference_value682 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b)) {
                                                                        self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101])
                                                                    } else {
                                                                        {
                                                                            let creation = self::ml_array::includes(&{ let ml_reference_value683 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101]), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])]))).borrow().clone(); ml_reference_value683 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b);
                                                                            {
                                                                                let active = (!creation && !(self::ml_array::includes(&{ let ml_reference_value684 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])]))).borrow().clone(); ml_reference_value684 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b)));
                                                                                if (creation && ((((shared_constructor_field5 != (0f64)) || (shared_constructor_field10 != (0f64))) || {
                                                                                    let source_ml_source2_s8562 = shared_constructor_field6.clone();
                                                                                    match source_ml_source2_s8562.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                }) || {
                                                                                    let source_ml_source2_s8565 = shared_constructor_field12.clone();
                                                                                    match source_ml_source2_s8565.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                })) {
                                                                                    self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 101, 115, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 115, 101, 108, 101, 99, 116, 105, 110, 103, 32, 97, 110, 121, 32, 105, 110, 112, 117, 116])
                                                                                } else {
                                                                                    if (active && ((shared_constructor_field5 > ({ let ml_reference_array685 = shared_constructor_field2.clone(); let ml_reference_projection685 = ml_reference_array685.borrow().len() as f64; ml_reference_projection685 })) || !(self::test_java_script_semantic_same_value(shared_constructor_field6.clone(), if (shared_constructor_field5 == (0f64)) {
                                                                                        shared_constructor_field1.clone()
                                                                                    } else {
                                                                                        ({
                                                                                            let source_ml_source2_array_read_value8577 = shared_constructor_field2.clone();
                                                                                            {
                                                                                                let source_ml_source2_array_read_index8578 = (shared_constructor_field5 - 1f64);
                                                                                                { let ml_reference_array686 = source_ml_source2_array_read_value8577.clone(); let ml_reference_position686 = source_ml_source2_array_read_index8578; let ml_reference_projection686 = self::ml_array::optional_at(&*ml_reference_array686.borrow(), self::ml_array::number_index(ml_reference_position686)); ml_reference_projection686 }
                                                                                            }
                                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                                                    })))) {
                                                                                        self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116])
                                                                                    } else {
                                                                                        if ((active && {
                                                                                            let source_ml_source2_s8580 = shared_constructor_field6.clone();
                                                                                            match source_ml_source2_s8580.clone() {
                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                    false
                                                                                                }
                                                                                                _ => {
                                                                                                    true
                                                                                                }
                                                                                            }
                                                                                        }) && !(((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && !shared_constructor_field11))) {
                                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 32, 115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 119, 114, 105, 116, 101, 32, 97, 99, 99, 101, 112, 116, 115, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 116, 101, 109])
                                                                                        } else {
                                                                                            if (self::ml_array::includes(&{ let ml_reference_value687 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value687 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && !shared_constructor_field11) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 100, 101, 99, 105, 115, 105, 111, 110])
                                                                                            } else {
                                                                                                if ((self::ml_array::includes(&{ let ml_reference_value688 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value688 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && shared_constructor_field11) && ((shared_constructor_field8 > ((9007199254740991f64 - shared_constructor_field7))) || (shared_constructor_field10 != ((shared_constructor_field7 + shared_constructor_field9))))) {
                                                                                                    self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 99, 104, 101, 99, 107, 115, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 115, 97, 118, 101, 100, 32, 115, 112, 114, 101, 97, 100, 32, 108, 101, 110, 103, 116, 104, 32, 98, 101, 102, 111, 114, 101, 32, 105, 110, 100, 101, 120, 101, 100, 32, 119, 111, 114, 107])
                                                                                                } else {
                                                                                                    if ((self::ml_array::includes(&{ let ml_reference_value689 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value689 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && shared_constructor_field11) && (shared_constructor_field9 >= shared_constructor_field8)) {
                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 105, 110, 112, 117, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                                                    } else {
                                                                                                        if (((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && !shared_constructor_field11) && (((shared_constructor_field10 != shared_constructor_field7) || (shared_constructor_field10 >= (9007199254740991f64))) || !(self::test_java_script_semantic_same_value(shared_constructor_field13.clone(), shared_constructor_field6.clone())))) {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 115, 99, 97, 108, 97, 114, 32, 119, 114, 105, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 109, 32, 97, 110, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                                                        } else {
                                                                                                            {
                                                                                                                let source_ml_source2_s8633 = if (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                                                                                    if (((match (match state.clone() {
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8636, _, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value8636.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                        }
                                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match (match state.clone() {
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8639, _, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value8639.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value8640, _) => {
                                                                                                                            source_ml_source2_data_property_value8640.clone()
                                                                                                                        }
                                                                                                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8641, _, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value8641.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value8642, _, _, _) => {
                                                                                                                            let source_ml_source2_data_property_value8642 = *source_ml_source2_data_property_value8642;
                                                                                                                            source_ml_source2_data_property_value8642.clone()
                                                                                                                        }
                                                                                                                    }, shared_constructor_field14.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8644, _, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value8644.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value8645, _, _) => {
                                                                                                                            let source_ml_source2_data_property_value8645 = *source_ml_source2_data_property_value8645;
                                                                                                                            source_ml_source2_data_property_value8645.clone()
                                                                                                                        }
                                                                                                                    }, shared_constructor_field4.clone()))) {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 110, 117, 109, 98, 101, 114, 32, 104, 105, 110, 116]))
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8647, _, _, _, _, _, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value8647.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                                }
                                                                                                                            }).clone().expect("a proven present value"));
                                                                                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Return(error.clone())
                                                                                                                            } else {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Next
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if !(match (match state.clone() {
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8648, _, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value8648.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                        }
                                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 99, 116, 105, 118, 101, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 99, 97, 108, 97, 114, 32, 99, 104, 105, 108, 100]))
                                                                                                                    } else {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Next
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source2_s8633.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Next => {
                                                                                                                        if ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]))) && {
                                                                                                                            let source_ml_source2_s8652 = shared_constructor_field14.clone();
                                                                                                                            match source_ml_source2_s8652.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }) {
                                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 99, 97, 108, 97, 114])
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let source_ml_source2_s8665 = if !(match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8666, _, _, _, _, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value8666.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                                    {
                                                                                                                                        let error_2 = self::read_java_script_array_species_state_error(heap.clone(), directory.clone(), (match state.clone() {
                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8667, _, _, _, _, _) => {
                                                                                                                                                ((source_ml_source2_data_property_value8667.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value"));
                                                                                                                                        if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Return(error_2.clone())
                                                                                                                                        } else {
                                                                                                                                            if (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8668, _, _, _, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value8668.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(source_ml_source2_data_property_value8669, _, _, _, _, _, _, _, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value8669 = *source_ml_source2_data_property_value8669;
                                                                                                                                                    source_ml_source2_data_property_value8669.clone()
                                                                                                                                                }
                                                                                                                                            }, shared_constructor_field1.clone())) || ((match (match state.clone() {
                                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8673, _, _, _, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value8673.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(_, source_ml_source2_data_property_value8674, _, _, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value8674
                                                                                                                                                }
                                                                                                                                            }) != (0f64))) {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 65, 114, 114, 97, 121, 83, 112, 101, 99, 105, 101, 115, 67, 114, 101, 97, 116, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 122, 101, 114, 111, 32, 108, 101, 110, 103, 116, 104]))
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Next
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Next
                                                                                                                                };
                                                                                                                                match source_ml_source2_s8665.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Next => {
                                                                                                                                        if ((shared_constructor_field15 != (self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101]))) && match (match state.clone() {
                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8677, _, _, _, _, _) => {
                                                                                                                                                ((source_ml_source2_data_property_value8677.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110])
                                                                                                                                        } else {
                                                                                                                                            if (!creation && {
                                                                                                                                                let source_ml_source2_s8679 = shared_constructor_field12.clone();
                                                                                                                                                match source_ml_source2_s8679.clone() {
                                                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                                        false
                                                                                                                                                    }
                                                                                                                                                    _ => {
                                                                                                                                                        true
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }) {
                                                                                                                                                self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 101, 100, 32, 114, 101, 115, 117, 108, 116, 32, 79, 98, 106, 101, 99, 116])
                                                                                                                                            } else {
                                                                                                                                                if ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) && (match (match state.clone() {
                                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8685, _, _, _, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value8685.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value((match state.clone() {
                                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8686, _, _, _, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value8686.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().expect("a proven present value"), match directory.clone() {
                                                                                                                                                    self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, source_ml_source2_data_property_value8687, _, _, _) => {
                                                                                                                                                        let source_ml_source2_data_property_value8687 = *source_ml_source2_data_property_value8687;
                                                                                                                                                        source_ml_source2_data_property_value8687.clone()
                                                                                                                                                    }
                                                                                                                                                })))) {
                                                                                                                                                    self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                                                                                } else {
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_s8727 = if (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                                                                                                                                            {
                                                                                                                                                                let constructor = if {
                                                                                                                                                                    let source_ml_source2_own_property_operands8730 = (state.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_own_property_key8731 = ((source_ml_source2_own_property_operands8730.clone()).1,);
                                                                                                                                                                        match (source_ml_source2_own_property_operands8730.clone()).0 {
                                                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field8732, source_ml_source2_owned_constructor_field8733, source_ml_source2_owned_constructor_field8734, source_ml_source2_owned_constructor_field8735, source_ml_source2_owned_constructor_field8736, source_ml_source2_owned_constructor_field8737, source_ml_source2_owned_constructor_field8738) => {
                                                                                                                                                                                (((((((((((((((((((((((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 108, 101, 75, 101, 121])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 80, 111, 115, 105, 116, 105, 111, 110])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 83, 116, 97, 114, 116])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field8732.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field8733.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field8734.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field8735.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field8736.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]))) && match source_ml_source2_owned_constructor_field8737.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key8731.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field8738.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } {
                                                                                                                                                                    match state.clone() {
                                                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8739, _, _, _) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value8739.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    None::<self::JavaScriptSemanticValue>
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let fact = match state.clone() {
                                                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8740, _, _) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value8740.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    {
                                                                                                                                                                        let classification = self::classify_java_script_array_species_constructor(heap.clone(), directory.clone(), constructor.clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                                                                                                                                        {
                                                                                                                                                                            let actual = self::read_java_script_known_array_constructor_fact(classification.clone());
                                                                                                                                                                            if ((((((((((match (constructor.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match constructor.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                                                                    false
                                                                                                                                                                                }
                                                                                                                                                                                _ => {
                                                                                                                                                                                    true
                                                                                                                                                                                }
                                                                                                                                                                            }) || match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match fact.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value8743, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8743
                                                                                                                                                                                }
                                                                                                                                                                            })) || match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(constructor.clone().expect("a proven present value"), ({
                                                                                                                                                                                let source_ml_source2_optional_chain_receiver8745 = (match state.clone() {
                                                                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8744, _, _, _, _, _) => {
                                                                                                                                                                                        ((source_ml_source2_data_property_value8744.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                                                                    }
                                                                                                                                                                                },);
                                                                                                                                                                                if match (source_ml_source2_optional_chain_receiver8745.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                                                                    Some({
                                                                                                                                                                                        let source_ml_source1_temporary_optional_chain322 = ((source_ml_source2_optional_chain_receiver8745.clone()).0).clone().expect("a proven present value");
                                                                                                                                                                                        match source_ml_source1_temporary_optional_chain322.clone() {
                                                                                                                                                                                            self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, source_ml_source2_data_property_value8746, _, _) => {
                                                                                                                                                                                                let source_ml_source2_data_property_value8746 = *source_ml_source2_data_property_value8746;
                                                                                                                                                                                                source_ml_source2_data_property_value8746.clone()
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    })
                                                                                                                                                                                } else {
                                                                                                                                                                                    None::<self::JavaScriptSemanticValue>
                                                                                                                                                                                }
                                                                                                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || (((((match constructor.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value8749, _) => {
                                                                                                                                                                                    Some((source_ml_source2_data_property_value8749.clone(),))
                                                                                                                                                                                }
                                                                                                                                                                                _ => {
                                                                                                                                                                                    None::<(self::ml::Big,)>
                                                                                                                                                                                }
                                                                                                                                                                            }).clone().expect("a proven present value")).0).clone()) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_data_property_value8750, _, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8750.clone()
                                                                                                                                                                                }
                                                                                                                                                                            }))) || ((match actual.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value8753, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8753
                                                                                                                                                                                }
                                                                                                                                                                            }) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value8754, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8754
                                                                                                                                                                                }
                                                                                                                                                                            }))) || ((match actual.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value8757, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8757.clone()
                                                                                                                                                                                }
                                                                                                                                                                            }) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value8758, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8758.clone()
                                                                                                                                                                                }
                                                                                                                                                                            }))) || ((match actual.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value8761, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8761
                                                                                                                                                                                }
                                                                                                                                                                            }) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value8762, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value8762
                                                                                                                                                                                }
                                                                                                                                                                            }))) || {
                                                                                                                                                                                let source_ml_source2_comparison_operand8765 = match actual.clone().expect("a proven present value") {
                                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value8763) => {
                                                                                                                                                                                        ((source_ml_source2_data_property_value8763.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                                                                    }
                                                                                                                                                                                };
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_comparison_operand8766 = match fact.clone().expect("a proven present value") {
                                                                                                                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value8764) => {
                                                                                                                                                                                            ((source_ml_source2_data_property_value8764.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    !(match (source_ml_source2_comparison_operand8765.clone(), source_ml_source2_comparison_operand8766.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                                                                                                                                                                                }
                                                                                                                                                                            }) {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 99, 116]))
                                                                                                                                                                            } else {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Next
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Next
                                                                                                                                                        };
                                                                                                                                                        match source_ml_source2_s8727.clone() {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Next => {
                                                                                                                                                                self::read_java_script_array_concatenation_child_error(heap.clone(), state.clone())
                                                                                                                                                            }
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Return(source_ml_source1_temporary_result327) => {
                                                                                                                                                                source_ml_source1_temporary_result327.clone()
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Return(source_ml_source1_temporary_result327_2) => {
                                                                                                                                        source_ml_source1_temporary_result327_2.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Return(source_ml_source1_temporary_result327_3) => {
                                                                                                                        source_ml_source1_temporary_result327_3.clone()
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
                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Return(source_ml_source1_temporary_result327_4) => {
                                                                source_ml_source1_temporary_result327_4.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Return(source_ml_source1_temporary_result327_5) => {
                                            source_ml_source1_temporary_result327_5.clone()
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
