// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result;
use super::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_read_java_script_call_template_directory_error_loop161;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_java_script_call_template_directory_error_loop158(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut owner: self::ml::Big,
    mut ordinal: f64,
    mut instructions: f64,
    mut captures: f64,
    mut source_ml_temporary_values153: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>,
    >,
    mut source_ml_temporary_index153: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result {
    loop {
        return if source_ml_temporary_index153
            < ({
                let ml_reference_array197 = source_ml_temporary_values153.clone();
                self::ml::Big::from_u128(ml_reference_array197.borrow().len() as u128)
            }) {
            {
                let template = {
                    let ml_reference_array198 = source_ml_temporary_values153.clone();
                    let ml_reference_position198 = source_ml_temporary_index153.clone();
                    self::ml_array::at(
                        &*ml_reference_array198.borrow(),
                        ml_reference_position198.to_index(),
                    )
                };
                if ({
                    let source_ml_source1_comparison_operand22646 = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
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
                            source_ml_source1_data_property_value22645,
                            _,
                        ) => ((source_ml_source1_data_property_value22645
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    {
                        let source_ml_source1_comparison_operand22647 = None::<()>;
                        !(match (
                            source_ml_source1_comparison_operand22646,
                            source_ml_source1_comparison_operand22647,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    }
                } && {
                    let source_ml_source1_comparison_operand22649 = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
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
                            source_ml_source1_data_property_value22648,
                            _,
                        ) => ((source_ml_source1_data_property_value22648
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    !(match (
                        source_ml_source1_comparison_operand22649,
                        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }) && {
                    let source_ml_source1_comparison_operand22652 = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
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
                            source_ml_source1_data_property_value22651,
                            _,
                        ) => ((source_ml_source1_data_property_value22651
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    !(match (
                        source_ml_source1_comparison_operand22652,
                        self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                } {
                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 114, 114, 111, 119, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                } else if (((((match template.clone() {
                    self::JavaScriptCallTemplate::CallTemplate(
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
                } || (({
                    let source_ml_source1_type_query_operand22669 = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value22668,
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
                        ) => source_ml_source1_data_property_value22668.clone(),
                    };
                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 105, 103, 105, 110, 116,
                ]))))
                    || ((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value22671,
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
                        ) => source_ml_source1_data_property_value22671.clone(),
                    }) <= (self::ml::Big::from_i128(0))))
                    || ((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value22673,
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
                        ) => source_ml_source1_data_property_value22673.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || !(self::test_java_script_call_code_coordinate(
                        match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                _,
                                source_ml_source1_data_property_value22674,
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
                            ) => source_ml_source1_data_property_value22674,
                        },
                        4096f64,
                    )))
                    || ((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value22676,
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
                        ) => source_ml_source1_data_property_value22676.clone(),
                    }) < owner))
                    || (((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value22678,
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
                        ) => source_ml_source1_data_property_value22678.clone(),
                    }) == owner)
                        && ((match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                _,
                                source_ml_source1_data_property_value22680,
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
                            ) => source_ml_source1_data_property_value22680,
                        }) <= ordinal))
                {
                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 32, 111, 114, 100, 105, 110, 97, 108, 115]))
                } else {
                    {
                        let owner_2 = match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                source_ml_source1_data_property_value22681,
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
                            ) => source_ml_source1_data_property_value22681.clone(),
                        };
                        {
                            let ordinal_2 = match template.clone() {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    _,
                                    source_ml_source1_data_property_value22682,
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
                                ) => source_ml_source1_data_property_value22682,
                            };
                            {
                                let fn_ = {
                                    let source_ml_source1_array_read_value22684 =
                                        match program.clone() {
                                            self::JavaScriptCallProgram::CallProgram(
                                                source_ml_source1_data_property_value22683,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22683.clone(),
                                        };
                                    {
                                        let source_ml_source1_array_read_index22685 =
                                            self::find_java_script_call_code_function(
                                                program.clone(),
                                                owner_2.clone(),
                                            );
                                        {
                                            let ml_reference_array199 =
                                                source_ml_source1_array_read_value22684;
                                            let ml_reference_position199 =
                                                source_ml_source1_array_read_index22685;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array199.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position199,
                                                ),
                                            )
                                        }
                                    }
                                };
                                if (match (fn_.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || ((match fn_.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        source_ml_source1_data_property_value22688,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value22688.clone(),
                                }) != (match template.clone() {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        _,
                                        source_ml_source1_data_property_value22689,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value22689.clone(),
                                }))) || !(self::ml_array::includes(
                                    &{
                                        (match program.clone() {
                                            self::JavaScriptCallProgram::CallProgram(
                                                _,
                                                source_ml_source1_data_property_value22691,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22691.clone(),
                                        })
                                        .borrow()
                                        .clone()
                                    },
                                    &match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            source_ml_source1_data_property_value22692,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value22692.clone(),
                                    },
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )) {
                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![97, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 100, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                } else if ((((!(self::test_java_script_call_code_coordinate(
                                    match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value22693,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value22693,
                                    },
                                    4097f64,
                                ))
                                    || !(self::test_java_script_call_code_coordinate(
                                        match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value22694,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22694,
                                        },
                                        4097f64,
                                    )))
                                    || !(self::test_java_script_call_code_coordinate(
                                        match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value22695,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22695,
                                        },
                                        4097f64,
                                    )))
                                    || ((match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value22697,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value22697,
                                    }) == 0f64))
                                    || (((match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value22701,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value22701,
                                    } + match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value22702,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value22702,
                                    }) + 1f64)
                                        > (match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value22703,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22703,
                                        })))
                                    || !(self::test_java_script_call_code_coordinate(
                                        match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value22704,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22704,
                                        },
                                        (match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value22705,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value22705,
                                        } + 1f64),
                                    ))
                                {
                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 44, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 110, 101, 101, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                } else {
                                    {
                                        let captures_2 = captures
                                            + match template.clone() {
                                                self::JavaScriptCallTemplate::CallTemplate(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value22706,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source1_data_property_value22706,
                                            };
                                        {
                                            let instructions_2 = instructions + {
                                                let ml_reference_array201 = match template.clone() {
                                                    self::JavaScriptCallTemplate::CallTemplate(
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        source_ml_source1_data_property_value22707,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source1_data_property_value22707
                                                        .clone(),
                                                };
                                                ml_reference_array201.borrow().len() as f64
                                            };
                                            if ((captures_2 > 1_048_576f64)
                                                || (instructions_2 > 65_536f64))
                                                || (({
                                                    let ml_reference_array202 = match template.clone() {
                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, source_ml_source1_data_property_value22709, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value22709.clone()
                                                        }
                                                    };
                                                    ml_reference_array202.borrow().len() as f64
                                                }) == 0f64)
                                            {
                                                self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 99, 111, 100, 101, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 97, 116, 105, 99, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                {
                                                    let source_ml_temporary_values154 = match template.clone() {
                                                                self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, source_ml_source1_data_property_value22710, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value22710.clone()
                                                                }
                                                            };
                                                    {
                                                        let source_ml_temporary_index154 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s22711 = self::source_ml_temporary_read_java_script_call_template_directory_error_loop161(template.clone(), source_ml_temporary_values154, source_ml_temporary_index154);
                                                            match source_ml_source1_s22711 {
                                                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Done => {
                                                                            {
                                                                                let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match template.clone() {
                                                                                    self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source1_data_property_value22712, _, _, _, _) => {
                                                                                        let source_ml_source1_data_property_value22712 = *source_ml_source1_data_property_value22712;
                                                                                        source_ml_source1_data_property_value22712.clone()
                                                                                    }
                                                                                });
                                                                                {
let ml_library_inequality_2 = prototype_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(prototype_error.clone())
                                                                                } else
                                                                                    if {
                                                                                        let source_ml_source1_s22714 = match template.clone() {
                                                                                            self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source1_data_property_value22724, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value22724 = *source_ml_source1_data_property_value22724;
                                                                                                source_ml_source1_data_property_value22724.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source1_s22714 {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    } || !(match match match template.clone() {
                                                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source1_data_property_value22725, _, _, _, _) => {
                                                                                            let source_ml_source1_data_property_value22725 = *source_ml_source1_data_property_value22725;
                                                                                            source_ml_source1_data_property_value22725.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                            None::<bool>
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value22726) => {
                                                                                            Some(source_ml_source1_data_property_value22726)
                                                                                        }
                                                                                    } { Some(ml_present) => ml_present, None => false }) {
                                                                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                                    } else {
                                                                                        {
                                                                                            let name_error = self::read_java_script_ordinary_value_error(heap.clone(), match template.clone() {
                                                                                                self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value22727, _, _, _) => {
                                                                                                    let source_ml_source1_data_property_value22727 = *source_ml_source1_data_property_value22727;
                                                                                                    source_ml_source1_data_property_value22727.clone()
                                                                                                }
                                                                                            });
                                                                                            {
let ml_library_inequality_1 = name_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                                self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(name_error.clone())
                                                                                            } else if ({
                                                                                                    let source_ml_source1_s22729 = match template.clone() {
                                                                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value22739, _, _, _) => {
                                                                                                            let source_ml_source1_data_property_value22739 = *source_ml_source1_data_property_value22739;
                                                                                                            source_ml_source1_data_property_value22739.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source1_s22729 {
                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                            false
                                                                                                        }
                                                                                                        _ => {
                                                                                                            true
                                                                                                        }
                                                                                                    }
                                                                                                } && {
                                                                                                    let source_ml_source1_s22741 = match template.clone() {
                                                                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value22751, _, _, _) => {
                                                                                                            let source_ml_source1_data_property_value22751 = *source_ml_source1_data_property_value22751;
                                                                                                            source_ml_source1_data_property_value22751.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source1_s22741 {
                                                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                            false
                                                                                                        }
                                                                                                        _ => {
                                                                                                            true
                                                                                                        }
                                                                                                    }
                                                                                                }) || ((self::read_java_script_semantic_text_length(match template.clone() {
                                                                                                    self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value22753, _, _, _) => {
                                                                                                        let source_ml_source1_data_property_value22753 = *source_ml_source1_data_property_value22753;
                                                                                                        source_ml_source1_data_property_value22753.clone()
                                                                                                    }
                                                                                                })) > 65_536f64) {
                                                                                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_temporary_index153_2 = source_ml_temporary_index153.add(&self::ml::Big::from_i128(1));
                                                                                                        {
                                                                                                            (owner, ordinal, instructions, captures, source_ml_temporary_index153) = (owner_2.clone(), ordinal_2, instructions_2, captures_2, source_ml_temporary_index153_2);
                                                                                                            continue;
                                                                                                        }
                                                                                                    }
                                                                                                }
}
                                                                                        }
                                                                                    }
}

                                                                            }
                                                                        }
                                                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Return(source_ml_temporary_result161) => {
                                                                            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(source_ml_temporary_result161.clone())
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
            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Done
        };
    }
}
