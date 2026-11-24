// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result;
use super::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_bound_construction_operands;
use super::read_java_script_construction_completion_state_error;
use super::read_java_script_construction_fact;
use super::read_java_script_construction_mode;
use super::source_ml_source1_temporary_read_java_script_construction_state_error_loop974;
use super::source_ml_source1_temporary_read_java_script_construction_state_error_loop975;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_construction_state_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
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
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let invalid = self::read_java_script_array_species_directory_error(
                        heap.clone(),
                        directory.clone(),
                    );
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        invalid.clone()
                    } else {
                        if ((((((match state.clone() {
                            self::JavaScriptConstructionState::ConstructionState(
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
                        } || !(self::ml_array::includes(
                            &{
                                let ml_reference_value948 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            116, 97, 114, 103, 101, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 101, 119, 84, 97, 114, 103, 101, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116,
                                            114, 117, 99, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 118, 101, 114, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 112,
                                            114, 105, 109, 105, 116, 105, 118, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103,
                                            101, 116, 116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 102,
                                            97, 108, 108, 98, 97, 99, 107,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116, 32, 114, 111, 111, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 108, 108, 111, 99, 97, 116, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101,
                                            110, 116, 115,
                                        ]),
                                        self::ml_string::Text::from_units(vec![98, 111, 100, 121]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value948
                            },
                            &shared_constructor_field5.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))) || (({
                            let ml_reference_array949 = shared_constructor_field4.clone();
                            let ml_reference_projection949 =
                                ml_reference_array949.borrow().len() as f64;
                            ml_reference_projection949
                        }) > (1048576f64)))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field10)))
                            || {
                                let source_ml_source2_same_value_operand18610 =
                                    shared_constructor_field10;
                                {
                                    let source_ml_source2_same_value_operand18611 = (-0f64);
                                    (((source_ml_source2_same_value_operand18610
                                        == source_ml_source2_same_value_operand18611)
                                        && ((source_ml_source2_same_value_operand18610 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand18610)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand18611))))
                                        || ((source_ml_source2_same_value_operand18610
                                            != source_ml_source2_same_value_operand18610)
                                            && (source_ml_source2_same_value_operand18611
                                                != source_ml_source2_same_value_operand18611)))
                                }
                            })
                            || (shared_constructor_field10 < (0f64)))
                            || (shared_constructor_field10
                                > ({
                                    let ml_reference_array950 = shared_constructor_field4.clone();
                                    let ml_reference_projection950 =
                                        ml_reference_array950.borrow().len() as f64;
                                    ml_reference_projection950
                                })))
                        {
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114,
                                101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 114,
                                103, 117, 109, 101, 110, 116, 32, 118, 101, 99, 116, 111, 114, 32,
                                97, 110, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110,
                                116, 105, 110, 117, 97, 116, 105, 111, 110,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values972 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        vec![
                                            shared_constructor_field2.clone(),
                                            shared_constructor_field3.clone(),
                                            shared_constructor_field7.clone(),
                                            shared_constructor_field8.clone(),
                                            shared_constructor_field9.clone(),
                                        ],
                                        {
                                            let ml_reference_value951 =
                                                shared_constructor_field4.borrow().clone();
                                            ml_reference_value951
                                        },
                                    )),
                                );
                                {
                                    let source_ml_source1_temporary_index972 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s18624 = self::source_ml_source1_temporary_read_java_script_construction_state_error_loop974(heap.clone(), source_ml_source1_temporary_values972.clone(), source_ml_source1_temporary_index972.clone());
                                        match source_ml_source2_s18624.clone() {
                                                self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Done => {
                                                    {
                                                        let target = self::read_java_script_construction_fact(directory.clone(), shared_constructor_field2.clone(), Some(heap.clone()));
                                                        {
                                                            let new_target = self::read_java_script_construction_fact(directory.clone(), shared_constructor_field3.clone(), Some(heap.clone()));
                                                            if ((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116]))) && ((match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match target.clone().expect("a proven present value") {
                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value18629, _, _, _) => {
                                                                    source_ml_source2_data_property_value18629
                                                                }
                                                            })) || {
                                                                let source_ml_source2_comparison_operand18631 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_comparison_operand18632 = self::read_java_script_construction_mode(program.clone(), heap.clone(), target.clone().expect("a proven present value"));
                                                                    !(match (source_ml_source2_comparison_operand18631.clone(), source_ml_source2_comparison_operand18632.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false })
                                                                }
                                                            })) {
                                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 111, 114, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 107, 105, 110, 100])
                                                            } else {
                                                                if (!(self::ml_array::includes(&{ let ml_reference_value952 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116]), self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116])]))).borrow().clone(); ml_reference_value952 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b)) && (match (new_target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match new_target.clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value18634, _, _, _) => {
                                                                        source_ml_source2_data_property_value18634
                                                                    }
                                                                }))) {
                                                                    self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 105, 100, 97, 116, 101, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_values973 = std::rc::Rc::new(std::cell::RefCell::new(vec![target.clone(), new_target.clone()]));
                                                                        {
                                                                            let source_ml_source1_temporary_index973 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s18635 = self::source_ml_source1_temporary_read_java_script_construction_state_error_loop975(program.clone(), heap.clone(), source_ml_source1_temporary_values973.clone(), source_ml_source1_temporary_index973.clone());
                                                                                match source_ml_source2_s18635.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Done => {
                                                                                        if ((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))) || !(match (match state.clone() {
                                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18638) => {
                                                                                                ((source_ml_source2_data_property_value18638.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                            {
                                                                                                let operation = match state.clone() {
                                                                                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18639) => {
                                                                                                        ((source_ml_source2_data_property_value18639.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let operands = self::read_java_script_bound_construction_operands(heap.clone(), state.clone());
                                                                                                    if (match (operation.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || {
                                                                                                        let source_ml_source2_s18641 = match operation.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value18644, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value18644 = *source_ml_source2_data_property_value18644;
                                                                                                                source_ml_source2_data_property_value18644.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        match source_ml_source2_s18641.clone() {
                                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                false
                                                                                                            }
                                                                                                            _ => {
                                                                                                                true
                                                                                                            }
                                                                                                        }
                                                                                                    }) {
                                                                                                        self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 110, 111, 110, 45, 98, 111, 117, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                                                                                                    } else {
                                                                                                        {
                                                                                                            let actual = {
                                                                                                                let source_ml_source2_array_read_value18649 = shared_constructor_field1.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_array_read_index18650 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match match operation.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value18647, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                            let source_ml_source2_data_property_value18647 = *source_ml_source2_data_property_value18647;
                                                                                                                            source_ml_source2_data_property_value18647.clone()
                                                                                                                        }
                                                                                                                    } {
                                                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value18648, _) => {
                                                                                                                            Some((source_ml_source2_data_property_value18648.clone(),))
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            None::<(self::ml::Big,)>
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value")).0).clone());
                                                                                                                    { let ml_reference_array953 = source_ml_source2_array_read_value18649.clone(); let ml_reference_position953 = source_ml_source2_array_read_index18650; let ml_reference_projection953 = self::ml_array::optional_at(&*ml_reference_array953.borrow(), self::ml_array::number_index(ml_reference_position953)); ml_reference_projection953 }
                                                                                                                }
                                                                                                            };
                                                                                                            if ((((((((((((((((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))) || (shared_constructor_field6 != (self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110])))) || !(match (match operation.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18655) => {
                                                                                                                    ((source_ml_source2_data_property_value18655.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                                                                                                                }
                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match actual.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                                                    true
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    false
                                                                                                                }
                                                                                                            }) || (((({ let ml_reference_value954 = operands.borrow().clone(); ml_reference_value954 }).0).1) != (self::ml_string::Text::from_units(vec![])))) || !(self::test_java_script_semantic_same_value(match operation.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value18664, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value18664 = *source_ml_source2_data_property_value18664;
                                                                                                                    source_ml_source2_data_property_value18664.clone()
                                                                                                                }
                                                                                                            }, (({ let ml_reference_value955 = operands.borrow().clone(); ml_reference_value955 }).0).4))) || !(self::test_java_script_semantic_same_value(match operation.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptConstructionState::ConstructionState(_, source_ml_source2_data_property_value18665, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value18665 = *source_ml_source2_data_property_value18665;
                                                                                                                    source_ml_source2_data_property_value18665.clone()
                                                                                                                }
                                                                                                            }, (({ let ml_reference_value956 = operands.borrow().clone(); ml_reference_value956 }).0).2))) || (({ let ml_reference_array957 = match operation.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value18667, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value18667.clone()
                                                                                                                }
                                                                                                            }; let ml_reference_projection957 = ml_reference_array957.borrow().len() as f64; ml_reference_projection957 }) != ({ let ml_reference_array959 = (({ let ml_reference_value958 = operands.borrow().clone(); ml_reference_value958 }).0).0; let ml_reference_projection959 = ml_reference_array959.borrow().len() as f64; ml_reference_projection959 }))) || { let ml_values960 = { let ml_reference_value961 = (match operation.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value18668, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value18668.clone()
                                                                                                                }
                                                                                                            }).borrow().clone(); ml_reference_value961 }; let ml_callback960 = { let operands = operands.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                                                                                let source_ml_source2_array_read_value18669 = (({ let ml_reference_value962 = operands.borrow().clone(); ml_reference_value962 }).0).0;
                                                                                                                {
                                                                                                                    let source_ml_source2_array_read_index18670 = index;
                                                                                                                    { let ml_reference_array963 = source_ml_source2_array_read_value18669.clone(); let ml_reference_position963 = source_ml_source2_array_read_index18670; let ml_reference_projection963 = self::ml_array::optional_at(&*ml_reference_array963.borrow(), self::ml_array::number_index(ml_reference_position963)); ml_reference_projection963 }
                                                                                                                }
                                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values960.iter().cloned().enumerate().any(|(ml_index960, ml_value960)| ml_callback960(ml_value960, (ml_index960 as f64))) }) || !(match (match state.clone() {
                                                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18671, _, _) => {
                                                                                                                    ((source_ml_source2_data_property_value18671.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                                                }
                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || (shared_constructor_field11 != (self::ml_string::Text::from_units(vec![])))) || {
                                                                                                                let source_ml_source2_s18675 = shared_constructor_field7.clone();
                                                                                                                match source_ml_source2_s18675.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Undefined => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            }) || {
                                                                                                                let source_ml_source2_s18678 = shared_constructor_field8.clone();
                                                                                                                match source_ml_source2_s18678.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Undefined => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            }) || {
                                                                                                                let source_ml_source2_s18681 = shared_constructor_field9.clone();
                                                                                                                match source_ml_source2_s18681.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Undefined => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            }) || (shared_constructor_field10 != (0f64))) {
                                                                                                                self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 110, 111, 110, 45, 98, 111, 117, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let call = match state.clone() {
                                                                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18685, _) => {
                                                                                                                            ((source_ml_source2_data_property_value18685.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let selected = match operation.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18686, _) => {
                                                                                                                                ((source_ml_source2_data_property_value18686.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        if (((match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) != (match (selected.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || ((!(match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match (selected.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && ((((((((match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, _) => {
                                                                                                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 67, 97, 108, 108, 82, 101, 99, 111, 114, 100])
                                                                                                                            }
                                                                                                                        }) != (match selected.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, _) => {
                                                                                                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 67, 97, 108, 108, 82, 101, 99, 111, 114, 100])
                                                                                                                            }
                                                                                                                        })) || ((match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value18689, _) => {
                                                                                                                                source_ml_source2_data_property_value18689
                                                                                                                            }
                                                                                                                        }) != (match selected.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value18690, _) => {
                                                                                                                                source_ml_source2_data_property_value18690
                                                                                                                            }
                                                                                                                        }))) || !(self::test_java_script_semantic_same_value(match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(source_ml_source2_data_property_value18691, _, _, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value18691 = *source_ml_source2_data_property_value18691;
                                                                                                                                source_ml_source2_data_property_value18691.clone()
                                                                                                                            }
                                                                                                                        }, match selected.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(source_ml_source2_data_property_value18692, _, _, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value18692 = *source_ml_source2_data_property_value18692;
                                                                                                                                source_ml_source2_data_property_value18692.clone()
                                                                                                                            }
                                                                                                                        }))) || !(self::test_java_script_semantic_same_value(match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, source_ml_source2_data_property_value18693, _, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value18693 = *source_ml_source2_data_property_value18693;
                                                                                                                                source_ml_source2_data_property_value18693.clone()
                                                                                                                            }
                                                                                                                        }, match selected.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, source_ml_source2_data_property_value18694, _, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value18694 = *source_ml_source2_data_property_value18694;
                                                                                                                                source_ml_source2_data_property_value18694.clone()
                                                                                                                            }
                                                                                                                        }))) || !(self::test_java_script_semantic_same_value(match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, source_ml_source2_data_property_value18695) => {
                                                                                                                                let source_ml_source2_data_property_value18695 = *source_ml_source2_data_property_value18695;
                                                                                                                                source_ml_source2_data_property_value18695.clone()
                                                                                                                            }
                                                                                                                        }, match selected.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, source_ml_source2_data_property_value18696) => {
                                                                                                                                let source_ml_source2_data_property_value18696 = *source_ml_source2_data_property_value18696;
                                                                                                                                source_ml_source2_data_property_value18696.clone()
                                                                                                                            }
                                                                                                                        }))) || (({ let ml_reference_array964 = match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value18699, _, _) => {
                                                                                                                                source_ml_source2_data_property_value18699.clone()
                                                                                                                            }
                                                                                                                        }; let ml_reference_projection964 = ml_reference_array964.borrow().len() as f64; ml_reference_projection964 }) != ({ let ml_reference_array965 = match selected.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value18700, _, _) => {
                                                                                                                                source_ml_source2_data_property_value18700.clone()
                                                                                                                            }
                                                                                                                        }; let ml_reference_projection965 = ml_reference_array965.borrow().len() as f64; ml_reference_projection965 }))) || { let ml_values966 = { let ml_reference_value967 = (match call.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value18701, _, _) => {
                                                                                                                                source_ml_source2_data_property_value18701.clone()
                                                                                                                            }
                                                                                                                        }).borrow().clone(); ml_reference_value967 }; let ml_callback966 = { let selected = selected.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                                                                                            let source_ml_source2_array_read_value18703 = match selected.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value18702, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value18702.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_array_read_index18704 = index;
                                                                                                                                { let ml_reference_array968 = source_ml_source2_array_read_value18703.clone(); let ml_reference_position968 = source_ml_source2_array_read_index18704; let ml_reference_projection968 = self::ml_array::optional_at(&*ml_reference_array968.borrow(), self::ml_array::number_index(ml_reference_position968)); ml_reference_projection968 }
                                                                                                                            }
                                                                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values966.iter().cloned().enumerate().any(|(ml_index966, ml_value966)| ml_callback966(ml_value966, (ml_index966 as f64))) }))) {
                                                                                                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 108, 101, 103, 97, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
                                                                                                                        } else {
                                                                                                                            self::read_java_script_construction_state_error(program.clone(), heap.clone(), directory.clone(), operation.clone().expect("a proven present value"))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::read_java_script_construction_completion_state_error(program.clone(), heap.clone(), state.clone())
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Return(source_ml_source1_temporary_result975) => {
                                                                                        source_ml_source1_temporary_result975.clone()
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
                                                self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Return(source_ml_source1_temporary_result975_2) => {
                                                    source_ml_source1_temporary_result975_2.clone()
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
