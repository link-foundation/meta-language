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
                    let source_ml_source1_comparison_operand21336 = match template.clone() {
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
                            source_ml_source1_data_property_value21335,
                            _,
                        ) => ((source_ml_source1_data_property_value21335
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    {
                        let source_ml_source1_comparison_operand21337 = None::<()>;
                        !(match (
                            source_ml_source1_comparison_operand21336,
                            source_ml_source1_comparison_operand21337,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    }
                } && {
                    let source_ml_source1_comparison_operand21339 = match template.clone() {
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
                            source_ml_source1_data_property_value21338,
                            _,
                        ) => ((source_ml_source1_data_property_value21338
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    !(match (
                        source_ml_source1_comparison_operand21339,
                        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }) && {
                    let source_ml_source1_comparison_operand21342 = match template.clone() {
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
                            source_ml_source1_data_property_value21341,
                            _,
                        ) => ((source_ml_source1_data_property_value21341
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    !(match (
                        source_ml_source1_comparison_operand21342,
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
                    let source_ml_source1_type_query_operand21359 = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value21358,
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
                        ) => source_ml_source1_data_property_value21358.clone(),
                    };
                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 105, 103, 105, 110, 116,
                ]))))
                    || ((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value21361,
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
                        ) => source_ml_source1_data_property_value21361.clone(),
                    }) <= (self::ml::Big::from_i128(0))))
                    || ((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value21363,
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
                        ) => source_ml_source1_data_property_value21363.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || !(self::test_java_script_call_code_coordinate(
                        match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                _,
                                source_ml_source1_data_property_value21364,
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
                            ) => source_ml_source1_data_property_value21364,
                        },
                        4096f64,
                    )))
                    || ((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value21366,
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
                        ) => source_ml_source1_data_property_value21366.clone(),
                    }) < owner))
                    || (((match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value21368,
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
                        ) => source_ml_source1_data_property_value21368.clone(),
                    }) == owner)
                        && ((match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                _,
                                source_ml_source1_data_property_value21370,
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
                            ) => source_ml_source1_data_property_value21370,
                        }) <= ordinal))
                {
                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 32, 111, 114, 100, 105, 110, 97, 108, 115]))
                } else {
                    {
                        let owner_2 = match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                source_ml_source1_data_property_value21371,
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
                            ) => source_ml_source1_data_property_value21371.clone(),
                        };
                        {
                            let ordinal_2 = match template.clone() {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    _,
                                    source_ml_source1_data_property_value21372,
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
                                ) => source_ml_source1_data_property_value21372,
                            };
                            {
                                let fn_ = {
                                    let source_ml_source1_array_read_value21374 =
                                        match program.clone() {
                                            self::JavaScriptCallProgram::CallProgram(
                                                source_ml_source1_data_property_value21373,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21373.clone(),
                                        };
                                    {
                                        let source_ml_source1_array_read_index21375 =
                                            self::find_java_script_call_code_function(
                                                program.clone(),
                                                owner_2.clone(),
                                            );
                                        {
                                            let ml_reference_array199 =
                                                source_ml_source1_array_read_value21374;
                                            let ml_reference_position199 =
                                                source_ml_source1_array_read_index21375;
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
                                        source_ml_source1_data_property_value21378,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value21378.clone(),
                                }) != (match template.clone() {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        _,
                                        source_ml_source1_data_property_value21379,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value21379.clone(),
                                }))) || !(self::ml_array::includes(
                                    &{
                                        (match program.clone() {
                                            self::JavaScriptCallProgram::CallProgram(
                                                _,
                                                source_ml_source1_data_property_value21381,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21381.clone(),
                                        })
                                        .borrow()
                                        .clone()
                                    },
                                    &match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            source_ml_source1_data_property_value21382,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value21382.clone(),
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
                                            source_ml_source1_data_property_value21383,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value21383,
                                    },
                                    4097f64,
                                ))
                                    || !(self::test_java_script_call_code_coordinate(
                                        match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value21384,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21384,
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
                                                source_ml_source1_data_property_value21385,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21385,
                                        },
                                        4097f64,
                                    )))
                                    || ((match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value21387,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value21387,
                                    }) == 0f64))
                                    || (((match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value21391,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value21391,
                                    } + match template.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value21392,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value21392,
                                    }) + 1f64)
                                        > (match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value21393,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21393,
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
                                                source_ml_source1_data_property_value21394,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21394,
                                        },
                                        (match template.clone() {
                                            self::JavaScriptCallTemplate::CallTemplate(
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value21395,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value21395,
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
                                                    source_ml_source1_data_property_value21396,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source1_data_property_value21396,
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
                                                        source_ml_source1_data_property_value21397,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source1_data_property_value21397
                                                        .clone(),
                                                };
                                                ml_reference_array201.borrow().len() as f64
                                            };
                                            if ((captures_2 > 1_048_576f64)
                                                || (instructions_2 > 65_536f64))
                                                || (({
                                                    let ml_reference_array202 = match template.clone() {
                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, source_ml_source1_data_property_value21399, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value21399.clone()
                                                        }
                                                    };
                                                    ml_reference_array202.borrow().len() as f64
                                                }) == 0f64)
                                            {
                                                self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 99, 111, 100, 101, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 97, 116, 105, 99, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                {
                                                    let source_ml_temporary_values154 = match template.clone() {
                                                                self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, source_ml_source1_data_property_value21400, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value21400.clone()
                                                                }
                                                            };
                                                    {
                                                        let source_ml_temporary_index154 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s21401 = self::source_ml_temporary_read_java_script_call_template_directory_error_loop161(template.clone(), source_ml_temporary_values154, source_ml_temporary_index154);
                                                            match source_ml_source1_s21401 {
                                                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Done => {
                                                                            {
                                                                                let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match template.clone() {
                                                                                    self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source1_data_property_value21402, _, _, _, _) => {
                                                                                        let source_ml_source1_data_property_value21402 = *source_ml_source1_data_property_value21402;
                                                                                        source_ml_source1_data_property_value21402.clone()
                                                                                    }
                                                                                });
                                                                                {
let ml_library_inequality_2 = prototype_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(prototype_error.clone())
                                                                                } else
                                                                                    if {
                                                                                        let source_ml_source1_s21404 = match template.clone() {
                                                                                            self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source1_data_property_value21414, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value21414 = *source_ml_source1_data_property_value21414;
                                                                                                source_ml_source1_data_property_value21414.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source1_s21404 {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    } || !(match match match template.clone() {
                                                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source1_data_property_value21415, _, _, _, _) => {
                                                                                            let source_ml_source1_data_property_value21415 = *source_ml_source1_data_property_value21415;
                                                                                            source_ml_source1_data_property_value21415.clone()
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
                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value21416) => {
                                                                                            Some(source_ml_source1_data_property_value21416)
                                                                                        }
                                                                                    } { Some(ml_present) => ml_present, None => false }) {
                                                                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                                    } else {
                                                                                        {
                                                                                            let name_error = self::read_java_script_ordinary_value_error(heap.clone(), match template.clone() {
                                                                                                self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value21417, _, _, _) => {
                                                                                                    let source_ml_source1_data_property_value21417 = *source_ml_source1_data_property_value21417;
                                                                                                    source_ml_source1_data_property_value21417.clone()
                                                                                                }
                                                                                            });
                                                                                            {
let ml_library_inequality_1 = name_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                                self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(name_error.clone())
                                                                                            } else if ({
                                                                                                    let source_ml_source1_s21419 = match template.clone() {
                                                                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value21429, _, _, _) => {
                                                                                                            let source_ml_source1_data_property_value21429 = *source_ml_source1_data_property_value21429;
                                                                                                            source_ml_source1_data_property_value21429.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source1_s21419 {
                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                            false
                                                                                                        }
                                                                                                        _ => {
                                                                                                            true
                                                                                                        }
                                                                                                    }
                                                                                                } && {
                                                                                                    let source_ml_source1_s21431 = match template.clone() {
                                                                                                        self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value21441, _, _, _) => {
                                                                                                            let source_ml_source1_data_property_value21441 = *source_ml_source1_data_property_value21441;
                                                                                                            source_ml_source1_data_property_value21441.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source1_s21431 {
                                                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                            false
                                                                                                        }
                                                                                                        _ => {
                                                                                                            true
                                                                                                        }
                                                                                                    }
                                                                                                }) || ((self::read_java_script_semantic_text_length(match template.clone() {
                                                                                                    self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value21443, _, _, _) => {
                                                                                                        let source_ml_source1_data_property_value21443 = *source_ml_source1_data_property_value21443;
                                                                                                        source_ml_source1_data_property_value21443.clone()
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
