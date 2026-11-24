// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result;
use super::find_java_script_call_code_function;
use super::find_java_script_call_template;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_code_destination;
use super::read_java_script_executable_call_function;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_template_loop923(
    mut owner: self::ml::Big,
    mut ordinal: f64,
    mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array2829 = templates.clone();
                        let ml_reference_position2829 = middle;
                        let ml_reference_projection2829 = self::ml_array::optional_at(
                            &*ml_reference_array2829.borrow(),
                            self::ml_array::number_index(ml_reference_position2829),
                        );
                        ml_reference_projection2829
                    };
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Return((-1f64))
                    } else {
                        if (((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallTemplate::CallTemplate(
                                source_ml_source2_data_property_value40975,
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
                            ) => source_ml_source2_data_property_value40975.clone(),
                        }) == owner)
                            && ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    _,
                                    source_ml_source2_data_property_value40977,
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
                                ) => source_ml_source2_data_property_value40977,
                            }) == ordinal))
                        {
                            self::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Return(middle)
                        } else {
                            if (((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    source_ml_source2_data_property_value40979,
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
                                ) => source_ml_source2_data_property_value40979.clone(),
                            }) < owner)
                                || (((match entry.clone().expect("a proven present value") {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        source_ml_source2_data_property_value40981,
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
                                    ) => source_ml_source2_data_property_value40981.clone(),
                                }) == owner)
                                    && ((match entry.clone().expect("a proven present value") {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            source_ml_source2_data_property_value40983,
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
                                        ) => source_ml_source2_data_property_value40983,
                                    }) < ordinal)))
                            {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result {
    MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Done,
    MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Return(
        Option<self::JavaScriptCallFunction>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_executable_call_function_loop932(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut target: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (depth
                    <= ({
                        let ml_reference_array2830 = shared_constructor_field1.clone();
                        let ml_reference_projection2830 =
                            ml_reference_array2830.borrow().len() as f64;
                        ml_reference_projection2830
                    }))
                {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Object(current_identity, callable) => {
                            if !callable {
                                self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Return(None::<self::JavaScriptCallFunction>)
                            } else {
                                {
                                    let object = {
                                        let source_ml_source2_array_read_value40988 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_array_read_index40989 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), current_identity.clone());
                                            { let ml_reference_array2831 = source_ml_source2_array_read_value40988.clone(); let ml_reference_position2831 = source_ml_source2_array_read_index40989; let ml_reference_projection2831 = self::ml_array::optional_at(&*ml_reference_array2831.borrow(), self::ml_array::number_index(ml_reference_position2831)); ml_reference_projection2831 }
                                        }
                                    };
                                    if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    }) {
                                        {
                                            let next = (((match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source2_data_property_value40998, _, _) => {
                                                    let source_ml_source2_data_property_value40998 = *source_ml_source2_data_property_value40998;
                                                    Some((source_ml_source2_data_property_value40998.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::JavaScriptSemanticValue,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone();
                                            if match (next.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } {
                                                self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Return(None::<self::JavaScriptCallFunction>)
                                            } else {
                                                {
                                                    let depth_2 = (depth + 1f64);
                                                    {
                                                        (target, depth) = (next.clone(), depth_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let actual = self::read_java_script_executable_call_function(program.clone(), heap.clone(), current_identity.clone());
                                            self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Return(if match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                None::<self::JavaScriptCallFunction>
                                            } else {
                                                Some({
                                                    let source_ml_source2_constructor_field41001 = match actual.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value41000, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value41000.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field41003 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source2_constructor_field41004 = 0f64;
                                                            {
                                                                let source_ml_source2_constructor_field41005 = 2f64;
                                                                {
                                                                    let source_ml_source2_constructor_field41006 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                    {
                                                                        let source_ml_source2_constructor_field41007 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::BoundFunctionCall(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), source_ml_source2_constructor_field41001.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), Box::new(source_ml_source2_constructor_field41003.clone()), source_ml_source2_constructor_field41004, source_ml_source2_constructor_field41005, source_ml_source2_constructor_field41006.clone(), source_ml_source2_constructor_field41007.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                })
                                            })
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Return(None::<self::JavaScriptCallFunction>)
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result {
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Done,
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_template_directory_error_loop940(
    mut program: self::JavaScriptCallProgram,
    mut source_ml_source1_temporary_values937: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index937: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result {
    loop {
        return if (source_ml_source1_temporary_index937
            < ({
                let ml_reference_array2832 = source_ml_source1_temporary_values937.clone();
                let ml_reference_projection2832 =
                    self::ml::Big::from_u128(ml_reference_array2832.borrow().len() as u128);
                ml_reference_projection2832
            })) {
            {
                let object = {
                    let ml_reference_array2833 = source_ml_source1_temporary_values937.clone();
                    let ml_reference_position2833 = source_ml_source1_temporary_index937.clone();
                    let ml_reference_projection2833 = self::ml_array::at(
                        &*ml_reference_array2833.borrow(),
                        ml_reference_position2833.to_index(),
                    );
                    ml_reference_projection2833
                };
                if (match object.clone() {
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                        true
                    }
                    _ => false,
                } && ((self::find_java_script_call_code_function(
                    program.clone(),
                    (((match object.clone() {
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source2_data_property_value41019,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => Some((source_ml_source2_data_property_value41019.clone(),)),
                        _ => None::<(self::ml::Big,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0)
                        .clone(),
                )) >= (0f64)))
                {
                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Return(self::ml_string::Text::from_units(vec![97, 32, 114, 117, 110, 116, 105, 109, 101, 32, 98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 115, 116, 97, 116, 105, 99, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    {
                        let source_ml_source1_temporary_index937_2 =
                            source_ml_source1_temporary_index937.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index937 =
                                source_ml_source1_temporary_index937_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result {
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Done,
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_template_directory_error_loop939(
    mut program: self::JavaScriptCallProgram,
    mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>,
    mut source_ml_source1_temporary_values935: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index935: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result {
    loop {
        return if (source_ml_source1_temporary_index935
            < ({
                let ml_reference_array2834 = source_ml_source1_temporary_values935.clone();
                let ml_reference_projection2834 =
                    self::ml::Big::from_u128(ml_reference_array2834.borrow().len() as u128);
                ml_reference_projection2834
            })) {
            {
                let object = {
                    let ml_reference_array2835 = source_ml_source1_temporary_values935.clone();
                    let ml_reference_position2835 = source_ml_source1_temporary_index935.clone();
                    let ml_reference_projection2835 = self::ml_array::at(
                        &*ml_reference_array2835.borrow(),
                        ml_reference_position2835.to_index(),
                    );
                    ml_reference_projection2835
                };
                match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        identity,
                        _,
                        _,
                        _,
                        _,
                        owner,
                        ordinal,
                        captures,
                        _,
                        lexical_new_target,
                    ) => {
                        if ((self::find_java_script_call_code_function(
                            program.clone(),
                            identity.clone(),
                        )) >= (0f64))
                        {
                            self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Return(self::ml_string::Text::from_units(vec![97, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 97, 108, 115, 111, 32, 110, 97, 109, 101, 32, 97, 32, 115, 116, 97, 116, 105, 99, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 110, 116, 114, 121]))
                        } else {
                            {
                                let template = {
                                    let source_ml_source2_array_read_index41028 =
                                        self::find_java_script_call_template(
                                            program.clone(),
                                            owner.clone(),
                                            ordinal,
                                        );
                                    {
                                        let ml_reference_array2836 = templates.clone();
                                        let ml_reference_position2836 =
                                            source_ml_source2_array_read_index41028;
                                        let ml_reference_projection2836 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2836.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2836,
                                                ),
                                            );
                                        ml_reference_projection2836
                                    }
                                };
                                if (!(match (
                                    ((lexical_new_target.clone().unwrap_or_else(|| {
                                        (None::<self::JavaScriptSemanticValue>,)
                                    }))
                                    .0)
                                        .clone(),
                                    None::<()>,
                                ) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) && ((({
                                    let source_ml_source2_optional_chain_receiver41031 =
                                        (template.clone(),);
                                    match if match (source_ml_source2_optional_chain_receiver41031
                                        .clone())
                                    .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        Some({
                                            let source_ml_source1_temporary_optional_chain936 =
                                                ((source_ml_source2_optional_chain_receiver41031
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_temporary_optional_chain936
                                                .clone()
                                            {
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
                                                    source_ml_source2_data_property_value41032,
                                                    _,
                                                ) => ((source_ml_source2_data_property_value41032
                                                    .clone()
                                                    .unwrap_or_else(|| {
                                                        (None::<self::ml_string::Text>,)
                                                    }))
                                                .0)
                                                    .clone(),
                                            }
                                        })
                                    } else {
                                        None::<self::ml_string::Text>
                                    } {
                                        Some(ml_nested) => match ml_nested {
                                            Some(ml_present) => Some(ml_present),
                                            None => None,
                                        },
                                        None => None,
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| {
                                    self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108,
                                    ])
                                })) != (self::ml_string::Text::from_units(vec![
                                    108, 101, 120, 105, 99, 97, 108,
                                ])))) {
                                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 108, 111, 115, 117, 114, 101, 115, 32, 99, 97, 112, 116, 117, 114, 101, 32, 97, 110, 32, 101, 110, 99, 108, 111, 115, 105, 110, 103, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116]))
                                } else {
                                    if (match (template.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || ((match template
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallTemplate::CallTemplate(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value41034,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value41034,
                                    }) != ({
                                        let ml_reference_array2837 = captures.clone();
                                        let ml_reference_projection2837 =
                                            ml_reference_array2837.borrow().len() as f64;
                                        ml_reference_projection2837
                                    }))) {
                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 108, 105, 118, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 97, 112, 116, 117, 114, 101, 32, 118, 101, 99, 116, 111, 114]))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index935_2 =
                                                source_ml_source1_temporary_index935
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index935 =
                                                    source_ml_source1_temporary_index935_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        let source_ml_source1_temporary_index935_3 =
                            source_ml_source1_temporary_index935.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index935 =
                                source_ml_source1_temporary_index935_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result {
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Done,
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result {
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Done,
    MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_template_directory_error_loop941(
    mut template: self::JavaScriptCallTemplate,
    mut source_ml_source1_temporary_values934: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>,
    >,
    mut source_ml_source1_temporary_index934: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result {
    loop {
        return match template.clone() {
            self::JavaScriptCallTemplate::CallTemplate(
                _,
                _,
                _,
                shared_constructor_field1,
                _,
                shared_constructor_field2,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index934
                    < ({
                        let ml_reference_array2838 = source_ml_source1_temporary_values934.clone();
                        let ml_reference_projection2838 =
                            self::ml::Big::from_u128(ml_reference_array2838.borrow().len() as u128);
                        ml_reference_projection2838
                    }))
                {
                    {
                        let instruction = {
                            let ml_reference_array2839 =
                                source_ml_source1_temporary_values934.clone();
                            let ml_reference_position2839 =
                                source_ml_source1_temporary_index934.clone();
                            let ml_reference_projection2839 = self::ml_array::at(
                                &*ml_reference_array2839.borrow(),
                                ml_reference_position2839.to_index(),
                            );
                            ml_reference_projection2839
                        };
                        {
                            let destination =
                                self::read_java_script_call_code_destination(instruction.clone());
                            if ((destination == (0f64))
                                || ((destination > shared_constructor_field1)
                                    && (destination
                                        <= (shared_constructor_field1
                                            + shared_constructor_field2))))
                            {
                                self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 99, 111, 100, 101, 32, 99, 97, 110, 110, 111, 116, 32, 111, 118, 101, 114, 119, 114, 105, 116, 101, 32, 105, 116, 115, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111, 114, 32, 99, 97, 112, 116, 117, 114, 101, 32, 115, 108, 111, 116, 115]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index934_2 =
                                        source_ml_source1_temporary_index934
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index934 =
                                            source_ml_source1_temporary_index934_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Done
                }
            }
        };
    }
}
