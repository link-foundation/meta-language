// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result;
use super::find_java_script_call_code_function;
use super::find_java_script_call_template;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_code_destination;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_template_directory_error_loop159(
    mut program: self::JavaScriptCallProgram,
    mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>,
    mut source_ml_temporary_values155: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index155: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result {
    loop {
        return if source_ml_temporary_index155
            < ({
                let ml_reference_array191 = source_ml_temporary_values155.clone();
                self::ml::Big::from_u128(ml_reference_array191.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array192 = source_ml_temporary_values155.clone();
                    let ml_reference_position192 = source_ml_temporary_index155.clone();
                    self::ml_array::at(
                        &*ml_reference_array192.borrow(),
                        ml_reference_position192.to_index(),
                    )
                };
                match object {
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
                        if (self::find_java_script_call_code_function(
                            program.clone(),
                            identity.clone(),
                        )) >= 0f64
                        {
                            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Return(self::ml_string::Text::from_units(vec![97, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 97, 108, 115, 111, 32, 110, 97, 109, 101, 32, 97, 32, 115, 116, 97, 116, 105, 99, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 110, 116, 114, 121]))
                        } else {
                            {
                                let template = {
                                    let source_ml_source1_array_read_index22628 =
                                        self::find_java_script_call_template(
                                            program.clone(),
                                            owner.clone(),
                                            ordinal,
                                        );
                                    {
                                        let ml_reference_array193 = templates.clone();
                                        let ml_reference_position193 =
                                            source_ml_source1_array_read_index22628;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array193.borrow(),
                                            self::ml_array::number_index(ml_reference_position193),
                                        )
                                    }
                                };
                                if !(match (
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
                                    let source_ml_source1_optional_chain_receiver22631 =
                                        (template.clone(),);
                                    match if match (source_ml_source1_optional_chain_receiver22631
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
                                            let source_ml_temporary_optional_chain156 =
                                                ((source_ml_source1_optional_chain_receiver22631
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_temporary_optional_chain156 {
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
                                                    source_ml_source1_data_property_value22632,
                                                    _,
                                                ) => ((source_ml_source1_data_property_value22632
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
                                ]))) {
                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 108, 111, 115, 117, 114, 101, 115, 32, 99, 97, 112, 116, 117, 114, 101, 32, 97, 110, 32, 101, 110, 99, 108, 111, 115, 105, 110, 103, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116]))
                                } else if match (template.clone(), None::<()>) {
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
                                        source_ml_source1_data_property_value22634,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value22634,
                                }) != ({
                                    let ml_reference_array194 = captures.clone();
                                    ml_reference_array194.borrow().len() as f64
                                })) {
                                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 108, 105, 118, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 97, 112, 116, 117, 114, 101, 32, 118, 101, 99, 116, 111, 114]))
                                } else {
                                    {
                                        let source_ml_temporary_index155_2 =
                                            source_ml_temporary_index155
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_temporary_index155 =
                                                source_ml_temporary_index155_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                        let source_ml_temporary_index155_3 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_3;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                        let source_ml_temporary_index155_4 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_4;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                    ) => {
                        let source_ml_temporary_index155_5 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_5;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                        let source_ml_temporary_index155_6 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_6;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                        let source_ml_temporary_index155_7 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_7;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                        let source_ml_temporary_index155_8 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_8;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                        let source_ml_temporary_index155_9 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_9;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                        let source_ml_temporary_index155_10 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_10;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                        let source_ml_temporary_index155_11 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_11;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                        let source_ml_temporary_index155_12 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_12;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                        let source_ml_temporary_index155_13 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_13;
                            continue;
                        }
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                        let source_ml_temporary_index155_14 =
                            source_ml_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index155 = source_ml_temporary_index155_14;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result {
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Done,
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result {
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Done,
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_template_directory_error_loop161(
    mut template: self::JavaScriptCallTemplate,
    mut source_ml_temporary_values154: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>,
    >,
    mut source_ml_temporary_index154: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result {
    loop {
        return if source_ml_temporary_index154
            < ({
                let ml_reference_array195 = source_ml_temporary_values154.clone();
                self::ml::Big::from_u128(ml_reference_array195.borrow().len() as u128)
            }) {
            {
                let instruction = {
                    let ml_reference_array196 = source_ml_temporary_values154.clone();
                    let ml_reference_position196 = source_ml_temporary_index154.clone();
                    self::ml_array::at(
                        &*ml_reference_array196.borrow(),
                        ml_reference_position196.to_index(),
                    )
                };
                {
                    let destination = self::read_java_script_call_code_destination(instruction);
                    if (destination == 0f64)
                        || ((destination
                            > (match template.clone() {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value22638,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value22638,
                            }))
                            && (destination
                                <= (match template.clone() {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value22641,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value22641,
                                } + match template.clone() {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value22642,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value22642,
                                })))
                    {
                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 99, 111, 100, 101, 32, 99, 97, 110, 110, 111, 116, 32, 111, 118, 101, 114, 119, 114, 105, 116, 101, 32, 105, 116, 115, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111, 114, 32, 99, 97, 112, 116, 117, 114, 101, 32, 115, 108, 111, 116, 115]))
                    } else {
                        {
                            let source_ml_temporary_index154_2 =
                                source_ml_temporary_index154.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index154 = source_ml_temporary_index154_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop161Done
        };
    }
}
