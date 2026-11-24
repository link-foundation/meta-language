// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result;
use super::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result;
use super::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result;
use super::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_call_code_function;
use super::find_java_script_call_template;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_call_template_function;
use super::source_ml_temporary_read_java_script_call_template_directory_error_loop158;
use super::source_ml_temporary_read_java_script_call_template_directory_error_loop159;
use super::source_ml_temporary_read_java_script_call_template_directory_error_loop160;
use super::source_ml_temporary_read_java_script_executable_call_function_loop152;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_executable_call_function(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> Option<self::JavaScriptCallFunction> {
    {
        let static_index =
            self::find_java_script_call_code_function(program.clone(), identity.clone());
        if static_index >= 0f64 {
            {
                let source_ml_source1_array_read_value16322 = match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        source_ml_source1_data_property_value16321,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value16321.clone(),
                };
                {
                    let ml_reference_array81 = source_ml_source1_array_read_value16322;
                    let ml_reference_position81 = static_index;
                    self::ml_array::optional_at(
                        &*ml_reference_array81.borrow(),
                        self::ml_array::number_index(ml_reference_position81),
                    )
                }
            }
        } else {
            {
                let closure = {
                    let source_ml_source1_array_read_value16326 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value16324,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value16324.clone(),
                    };
                    {
                        let source_ml_source1_array_read_index16327 =
                            self::find_java_script_ordinary_object(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value16325,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value16325.clone(),
                                },
                                identity.clone(),
                            );
                        {
                            let ml_reference_array82 = source_ml_source1_array_read_value16326;
                            let ml_reference_position82 = source_ml_source1_array_read_index16327;
                            self::ml_array::optional_at(
                                &*ml_reference_array82.borrow(),
                                self::ml_array::number_index(ml_reference_position82),
                            )
                        }
                    }
                };
                if match (closure.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    None::<self::JavaScriptCallFunction>
                } else {
                    match closure.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_f16333,
                            source_ml_source1_f16334,
                            source_ml_source1_f16335,
                            source_ml_source1_f16336,
                            source_ml_source1_f16337,
                        ) => {
                            let source_ml_source1_f16336 = *source_ml_source1_f16336;
                            {
                                let template = {
                                    let source_ml_source1_array_read_value16513 = (match program
                                        .clone()
                                    {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value16512,
                                            _,
                                            _,
                                        ) => ((source_ml_source1_data_property_value16512
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<
                                                    std::rc::Rc<
                                                        std::cell::RefCell<
                                                            Vec<self::JavaScriptCallTemplate>,
                                                        >,
                                                    >,
                                                >,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .unwrap_or_else(|| {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptCallTemplate,
                                        >::new(
                                        )))
                                    });
                                    {
                                        let source_ml_source1_array_read_index16514 =
                                            self::find_java_script_call_template(
                                                program.clone(),
                                                source_ml_source1_f16333.clone(),
                                                source_ml_source1_f16334,
                                            );
                                        {
                                            let ml_reference_array83 =
                                                source_ml_source1_array_read_value16513;
                                            let ml_reference_position83 =
                                                source_ml_source1_array_read_index16514;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array83.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position83,
                                                ),
                                            )
                                        }
                                    }
                                };
                                if match (template.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || ((match template.clone().expect("a proven present value") {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value16516,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value16516,
                                }) != ({
                                    let ml_reference_array84 = source_ml_source1_f16335.clone();
                                    ml_reference_array84.borrow().len() as f64
                                })) {
                                    None::<self::JavaScriptCallFunction>
                                } else {
                                    Some(self::project_java_script_call_template_function(
                                        template.clone().expect("a proven present value"),
                                        identity.clone(),
                                        source_ml_source1_f16335.clone(),
                                        source_ml_source1_f16336.clone(),
                                        ((source_ml_source1_f16337.clone().unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                        .0)
                                            .clone(),
                                    ))
                                }
                            }
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            first,
                            _,
                            _,
                        ) => {
                            let first = *first;
                            if match (first.clone(), None::<()>) {
                                (_ml_left, Some(_ml_right)) => false,
                                _ => false,
                            } {
                                None::<self::JavaScriptCallFunction>
                            } else {
                                {
                                    let depth = 0f64;
                                    {
                                        let source_ml_source1_s16517 = self::source_ml_temporary_read_java_script_executable_call_function_loop152(program.clone(), heap.clone(), identity.clone(), first.clone(), depth);
                                        match source_ml_source1_s16517 {
                                            self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Done => {
                                                None::<self::JavaScriptCallFunction>
                                            }
                                            self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(source_ml_temporary_result152) => {
                                                source_ml_temporary_result152.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
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
                        ) => None::<self::JavaScriptCallFunction>,
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => None::<self::JavaScriptCallFunction>,
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            None::<self::JavaScriptCallFunction>
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => None::<self::JavaScriptCallFunction>,
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_template_directory_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    {
        let templates = (match program.clone() {
            self::JavaScriptCallProgram::CallProgram(
                _,
                _,
                _,
                source_ml_source1_data_property_value17508,
                _,
                _,
            ) => ((source_ml_source1_data_property_value17508
                .clone()
                .unwrap_or_else(|| {
                    (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)
                }))
            .0)
                .clone(),
        })
        .clone()
        .unwrap_or_else(|| {
            std::rc::Rc::new(std::cell::RefCell::new(
                Vec::<self::JavaScriptCallTemplate>::new(),
            ))
        });
        {
            let ml_product_condition_1 = {
                let ml_reference_array85 = templates.clone();
                ml_reference_array85.borrow().len() as f64
            };
            if ml_product_condition_1 {
                self::ml_string::Text::from_units(vec![
                    99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 101, 120,
                    99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105,
                    99, 105, 116, 32, 115, 116, 97, 116, 105, 99, 32, 100, 105, 114, 101, 99, 116,
                    111, 114, 121, 32, 98, 117, 100, 103, 101, 116,
                ])
            } else {
                {
                    let owner = self::ml::Big::from_i128(0);
                    {
                        let ordinal = -1f64;
                        {
                            let instructions = 0f64;
                            {
                                let captures = 0f64;
                                {
                                    let source_ml_temporary_index153 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s17509 = self::source_ml_temporary_read_java_script_call_template_directory_error_loop158(program.clone(), heap.clone(), owner, ordinal, instructions, captures, templates.clone(), source_ml_temporary_index153);
                                        match source_ml_source1_s17509 {
                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Done => {
                                            {
                                                let source_ml_temporary_values155 = match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17510, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value17510.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index155 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s17511 = self::source_ml_temporary_read_java_script_call_template_directory_error_loop159(program.clone(), templates.clone(), source_ml_temporary_values155, source_ml_temporary_index155);
                                                        match source_ml_source1_s17511 {
                                                            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Done => {
                                                                {
                                                                    let source_ml_temporary_values157 = match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17512, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value17512.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index157 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s17513 = self::source_ml_temporary_read_java_script_call_template_directory_error_loop160(program.clone(), source_ml_temporary_values157, source_ml_temporary_index157);
                                                                            match source_ml_source1_s17513 {
                                                                                self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Done => {
                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                }
                                                                                self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Return(source_ml_temporary_result160) => {
                                                                                    source_ml_temporary_result160.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Return(source_ml_temporary_result160_2) => {
                                                                source_ml_temporary_result160_2.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop158Return(source_ml_temporary_result160_3) => {
                                            source_ml_temporary_result160_3.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_closure_metadata_descriptor(
    value: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    )>,
> {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            false,
        );
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source1_spread17514 = { descriptor.borrow().clone() };
            {
                let source_ml_source1_spread_field17515 = (source_ml_source1_spread17514.clone()).0;
                {
                    let source_ml_source1_spread_field17516 =
                        (source_ml_source1_spread17514.clone()).1;
                    {
                        let source_ml_source1_spread_field17517 =
                            (source_ml_source1_spread17514.clone()).2;
                        {
                            let source_ml_source1_spread_field17518 =
                                (source_ml_source1_spread17514.clone()).3;
                            {
                                let source_ml_source1_spread_field17519 =
                                    (source_ml_source1_spread17514.clone()).4;
                                {
                                    let source_ml_source1_spread_field17520 =
                                        (source_ml_source1_spread17514.clone()).5;
                                    {
                                        let source_ml_source1_spread_field17521 =
                                            (source_ml_source1_spread17514.clone()).6;
                                        {
                                            let source_ml_source1_spread_field17522 =
                                                (source_ml_source1_spread17514.clone()).7;
                                            {
                                                let source_ml_source1_spread_field17523 =
                                                    (source_ml_source1_spread17514.clone()).8;
                                                {
                                                    let source_ml_source1_spread_field17524 =
                                                        (source_ml_source1_spread17514.clone()).9;
                                                    {
                                                        let source_ml_source1_spread_field17525 =
                                                            (source_ml_source1_spread17514.clone())
                                                                .10;
                                                        {
                                                            let source_ml_source1_spread_field17526 =
                                                                (source_ml_source1_spread17514
                                                                    .clone())
                                                                .11;
                                                            (
                                                                source_ml_source1_spread_field17515,
                                                                source_ml_source1_spread_field17516,
                                                                source_ml_source1_spread_field17517,
                                                                source_ml_source1_spread_field17518,
                                                                source_ml_source1_spread_field17519,
                                                                source_ml_source1_spread_field17520,
                                                                source_ml_source1_spread_field17521,
                                                                source_ml_source1_spread_field17522,
                                                                value.clone(),
                                                                source_ml_source1_spread_field17524,
                                                                source_ml_source1_spread_field17525,
                                                                source_ml_source1_spread_field17526,
                                                            )
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_call_closure_object(
    template: self::JavaScriptCallTemplate,
    identity: self::ml::Big,
    owner: self::ml::Big,
    ordinal: f64,
    captures: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    receiver: self::JavaScriptSemanticValue,
    new_target: Option<self::JavaScriptSemanticValue>,
) -> self::JavaScriptOrdinaryObject {
    {
        let source_ml_temporary_constructor_field163 = match template.clone() {
            self::JavaScriptCallTemplate::CallTemplate(
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source1_data_property_value17528,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value17528 =
                    *source_ml_source1_data_property_value17528;
                source_ml_source1_data_property_value17528.clone()
            }
        };
        {
            let source_ml_temporary_constructor_field168 =
                std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(
                    Vec::<self::JavaScriptSemanticValue>::new(),
                    { captures.borrow().clone() },
                )));
            if !(match (new_target.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) && (((match template.clone() {
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
                    source_ml_source1_data_property_value17530,
                    _,
                ) => ((source_ml_source1_data_property_value17530
                    .clone()
                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                .0)
                    .clone(),
            })
            .clone()
            .unwrap_or_else(|| {
                self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])
            })) == (self::ml_string::Text::from_units(vec![
                108, 101, 120, 105, 99, 97, 108,
            ]))) {
                {
                    let source_ml_source1_constructor_field17539 =
                        Some(new_target.clone().expect("a proven present value"));
                    {
                        let source_ml_source1_constructor_field17542 =
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    Box::new(self::JavaScriptSemanticValue::Text(
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                        ])),
                                    )),
                                    self::create_java_script_closure_metadata_descriptor(
                                        self::JavaScriptSemanticValue::Number(
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
                                                    source_ml_source1_data_property_value17540,
                                                    _,
                                                    _,
                                                ) => source_ml_source1_data_property_value17540,
                                            },
                                        ),
                                    ),
                                ),
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    Box::new(self::JavaScriptSemanticValue::Text(
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            110f64, 97f64, 109f64, 101f64,
                                        ])),
                                    )),
                                    self::create_java_script_closure_metadata_descriptor(
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
                                                source_ml_source1_data_property_value17541,
                                                _,
                                                _,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value17541 =
                                                    *source_ml_source1_data_property_value17541;
                                                source_ml_source1_data_property_value17541.clone()
                                            }
                                        },
                                    ),
                                ),
                            ]));
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            identity.clone(),
                            Box::new(source_ml_temporary_constructor_field163.clone()),
                            true,
                            true,
                            source_ml_source1_constructor_field17542,
                            owner.clone(),
                            ordinal,
                            source_ml_temporary_constructor_field168.clone(),
                            Box::new(receiver.clone()),
                            Some(source_ml_source1_constructor_field17539),
                        )
                    }
                }
            } else {
                {
                    let source_ml_source1_constructor_field17553 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                    std::cell::RefCell::new(vec![
                                        108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                    ]),
                                ))),
                                self::create_java_script_closure_metadata_descriptor(
                                    self::JavaScriptSemanticValue::Number(match template.clone() {
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
                                            source_ml_source1_data_property_value17551,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value17551,
                                    }),
                                ),
                            ),
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                    std::cell::RefCell::new(vec![110f64, 97f64, 109f64, 101f64]),
                                ))),
                                self::create_java_script_closure_metadata_descriptor(
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
                                            source_ml_source1_data_property_value17552,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source1_data_property_value17552 =
                                                *source_ml_source1_data_property_value17552;
                                            source_ml_source1_data_property_value17552.clone()
                                        }
                                    },
                                ),
                            ),
                        ]));
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        identity.clone(),
                        Box::new(source_ml_temporary_constructor_field163.clone()),
                        true,
                        true,
                        source_ml_source1_constructor_field17553,
                        owner.clone(),
                        ordinal,
                        source_ml_temporary_constructor_field168.clone(),
                        Box::new(receiver.clone()),
                        None::<(Option<self::JavaScriptSemanticValue>,)>,
                    )
                }
            }
        }
    }
}
