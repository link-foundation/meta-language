// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptClosureAllocation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Result;
use super::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result;
use super::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result;
use super::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result;
use super::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result;
use super::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_call_code_function;
use super::find_java_script_call_template;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_call_template_function;
use super::read_java_script_ordinary_heap_error;
use super::source_ml_source1_temporary_allocate_java_script_call_closure_loop953;
use super::source_ml_source1_temporary_allocate_java_script_call_closure_loop954;
use super::source_ml_source1_temporary_allocate_java_script_call_closure_loop955;
use super::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop938;
use super::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop939;
use super::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop940;
use super::source_ml_source1_temporary_read_java_script_executable_call_function_loop932;
use super::source_ml_source3_record_order;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_executable_call_function(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> Option<self::JavaScriptCallFunction> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let static_index =
                self::find_java_script_call_code_function(program.clone(), identity.clone());
            if (static_index >= (0f64)) {
                {
                    let source_ml_source2_array_read_value18338 = match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(
                            source_ml_source2_data_property_value18337,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18337.clone(),
                    };
                    {
                        let ml_reference_array928 = source_ml_source2_array_read_value18338.clone();
                        let ml_reference_position928 = static_index;
                        let ml_reference_projection928 = self::ml_array::optional_at(
                            &*ml_reference_array928.borrow(),
                            self::ml_array::number_index(ml_reference_position928),
                        );
                        ml_reference_projection928
                    }
                }
            } else {
                {
                    let closure = {
                        let source_ml_source2_array_read_value18342 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_array_read_index18343 =
                                self::find_java_script_ordinary_object(
                                    shared_constructor_field1.clone(),
                                    identity.clone(),
                                );
                            {
                                let ml_reference_array929 =
                                    source_ml_source2_array_read_value18342.clone();
                                let ml_reference_position929 =
                                    source_ml_source2_array_read_index18343;
                                let ml_reference_projection929 = self::ml_array::optional_at(
                                    &*ml_reference_array929.borrow(),
                                    self::ml_array::number_index(ml_reference_position929),
                                );
                                ml_reference_projection929
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
                                            let source_ml_source2_s18351 = self::source_ml_source1_temporary_read_java_script_executable_call_function_loop932(program.clone(), heap.clone(), identity.clone(), first.clone(), depth);
                                            match source_ml_source2_s18351.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Done => {
                                                        None::<self::JavaScriptCallFunction>
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Result::MlSource1TemporaryReadJavaScriptExecutableCallFunctionLoop932Return(source_ml_source1_temporary_result932) => {
                                                        source_ml_source1_temporary_result932.clone()
                                                    }
                                                }
                                        }
                                    }
                                }
                            }
                            _ => {
                                match closure.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        owner,
                                        ordinal,
                                        captures,
                                        lexical_receiver,
                                        lexical_new_target,
                                    ) => {
                                        let lexical_receiver = *lexical_receiver;
                                        {
                                            let template = {
                                                let source_ml_source2_array_read_value18358 = (match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source2_data_property_value18357, _, _) => {
                                                            ((source_ml_source2_data_property_value18357.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new())));
                                                {
                                                    let source_ml_source2_array_read_index18359 =
                                                        self::find_java_script_call_template(
                                                            program.clone(),
                                                            owner.clone(),
                                                            ordinal,
                                                        );
                                                    {
                                                        let ml_reference_array930 =
                                                            source_ml_source2_array_read_value18358
                                                                .clone();
                                                        let ml_reference_position930 =
                                                            source_ml_source2_array_read_index18359;
                                                        let ml_reference_projection930 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array930.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position930,
                                                                ),
                                                            );
                                                        ml_reference_projection930
                                                    }
                                                }
                                            };
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
                                                    source_ml_source2_data_property_value18361,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value18361,
                                            }) != ({
                                                let ml_reference_array931 = captures.clone();
                                                let ml_reference_projection931 =
                                                    ml_reference_array931.borrow().len() as f64;
                                                ml_reference_projection931
                                            }))) {
                                                None::<self::JavaScriptCallFunction>
                                            } else {
                                                Some(self::project_java_script_call_template_function(template.clone().expect("a proven present value"), identity.clone(), captures.clone(), lexical_receiver.clone(), ((lexical_new_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()))
                                            }
                                        }
                                    }
                                    _ => None::<self::JavaScriptCallFunction>,
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
pub fn read_java_script_call_template_directory_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let templates =
                (match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value18362,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value18362
                        .clone()
                        .unwrap_or_else(|| {
                            (None::<
                                std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>,
                            >,)
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
            if (({
                let ml_reference_array932 = templates.clone();
                let ml_reference_projection932 = ml_reference_array932.borrow().len() as f64;
                ml_reference_projection932
            }) > (4096f64))
            {
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
                        let ordinal = (-1f64);
                        {
                            let instructions = 0f64;
                            {
                                let captures = 0f64;
                                {
                                    let source_ml_source1_temporary_index933 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s18363 = self::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop938(program.clone(), heap.clone(), owner.clone(), ordinal, instructions, captures, templates.clone(), source_ml_source1_temporary_index933.clone());
                                        match source_ml_source2_s18363.clone() {
                                                self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Done => {
                                                    {
                                                        let source_ml_source1_temporary_values935 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source1_temporary_index935 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s18365 = self::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop939(program.clone(), templates.clone(), source_ml_source1_temporary_values935.clone(), source_ml_source1_temporary_index935.clone());
                                                                match source_ml_source2_s18365.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Done => {
                                                                        {
                                                                            let source_ml_source1_temporary_values937 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source1_temporary_index937 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source2_s18367 = self::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop940(program.clone(), source_ml_source1_temporary_values937.clone(), source_ml_source1_temporary_index937.clone());
                                                                                    match source_ml_source2_s18367.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Done => {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop940Return(source_ml_source1_temporary_result940) => {
                                                                                            source_ml_source1_temporary_result940.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop939Return(source_ml_source1_temporary_result940_2) => {
                                                                        source_ml_source1_temporary_result940_2.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(source_ml_source1_temporary_result940_3) => {
                                                    source_ml_source1_temporary_result940_3.clone()
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
        (
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
        ),
        Vec<f64>,
    )>,
> {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            false,
        );
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_spread18368 = {
                let ml_reference_value933 = descriptor.borrow().clone();
                ml_reference_value933
            };
            {
                let source_ml_source2_spread_field18369 =
                    ((source_ml_source2_spread18368.clone()).0).0;
                {
                    let source_ml_source2_spread_field18370 =
                        ((source_ml_source2_spread18368.clone()).0).1;
                    {
                        let source_ml_source2_spread_field18371 =
                            ((source_ml_source2_spread18368.clone()).0).2;
                        {
                            let source_ml_source2_spread_field18372 =
                                ((source_ml_source2_spread18368.clone()).0).3;
                            {
                                let source_ml_source2_spread_field18373 =
                                    ((source_ml_source2_spread18368.clone()).0).4;
                                {
                                    let source_ml_source2_spread_field18374 =
                                        ((source_ml_source2_spread18368.clone()).0).5;
                                    {
                                        let source_ml_source2_spread_field18375 =
                                            ((source_ml_source2_spread18368.clone()).0).6;
                                        {
                                            let source_ml_source2_spread_field18376 =
                                                ((source_ml_source2_spread18368.clone()).0).7;
                                            {
                                                let source_ml_source2_spread_field18377 =
                                                    ((source_ml_source2_spread18368.clone()).0).8;
                                                {
                                                    let source_ml_source2_spread_field18378 =
                                                        ((source_ml_source2_spread18368.clone()).0)
                                                            .9;
                                                    {
                                                        let source_ml_source2_spread_field18379 =
                                                            ((source_ml_source2_spread18368
                                                                .clone())
                                                            .0)
                                                                .10;
                                                        {
                                                            let source_ml_source2_spread_field18380 =
                                                                ((source_ml_source2_spread18368
                                                                    .clone())
                                                                .0)
                                                                    .11;
                                                            {
                                                                let source_ml_source3_record_field422 = source_ml_source2_spread_field18369;
                                                                {
                                                                    let source_ml_source3_record_field423 = source_ml_source2_spread_field18370;
                                                                    {
                                                                        let source_ml_source3_record_field424 = source_ml_source2_spread_field18371;
                                                                        {
                                                                            let source_ml_source3_record_field425 = source_ml_source2_spread_field18372;
                                                                            {
                                                                                let source_ml_source3_record_field426 = source_ml_source2_spread_field18373.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field427 = source_ml_source2_spread_field18374;
                                                                                    {
                                                                                        let source_ml_source3_record_field428 = source_ml_source2_spread_field18375.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field429 = source_ml_source2_spread_field18376;
                                                                                            {
                                                                                                let source_ml_source3_record_field430 = value.clone();
                                                                                                {
                                                                                                    let source_ml_source3_record_field431 = source_ml_source2_spread_field18378;
                                                                                                    {
                                                                                                        let source_ml_source3_record_field432 = source_ml_source2_spread_field18379;
                                                                                                        {
                                                                                                            let source_ml_source3_record_field433 = source_ml_source2_spread_field18380;
                                                                                                            ((source_ml_source3_record_field422, source_ml_source3_record_field423, source_ml_source3_record_field424, source_ml_source3_record_field425, source_ml_source3_record_field426.clone(), source_ml_source3_record_field427, source_ml_source3_record_field428.clone(), source_ml_source3_record_field429, source_ml_source3_record_field430.clone(), source_ml_source3_record_field431, source_ml_source3_record_field432, source_ml_source3_record_field433), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], self::source_ml_source3_record_order::merge_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![self::source_ml_source3_record_order::rebase_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], (source_ml_source2_spread18368.clone()).1.clone()), vec![8f64]])))
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
    match template.clone() {
        self::JavaScriptCallTemplate::CallTemplate(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let source_ml_source1_temporary_constructor_field943 =
                    shared_constructor_field1.clone();
                {
                    let source_ml_source1_temporary_constructor_field948 =
                        std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(
                            Vec::<self::JavaScriptSemanticValue>::new(),
                            {
                                let ml_reference_value934 = captures.borrow().clone();
                                ml_reference_value934
                            },
                        )));
                    if (!(match (new_target.clone(), None::<()>) {
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
                            source_ml_source2_data_property_value18384,
                            _,
                        ) => ((source_ml_source2_data_property_value18384
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
                    ])))) {
                        {
                            let source_ml_source2_constructor_field18393 =
                                Some(new_target.clone().expect("a proven present value"));
                            {
                                let source_ml_source2_constructor_field18396 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            Box::new(self::JavaScriptSemanticValue::Text(
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                                ])),
                                            )),
                                            self::create_java_script_closure_metadata_descriptor(
                                                self::JavaScriptSemanticValue::Number(
                                                    shared_constructor_field3,
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
                                                shared_constructor_field2.clone(),
                                            ),
                                        ),
                                    ]));
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    identity.clone(),
                                    Box::new(
                                        source_ml_source1_temporary_constructor_field943.clone(),
                                    ),
                                    true,
                                    true,
                                    source_ml_source2_constructor_field18396.clone(),
                                    owner.clone(),
                                    ordinal,
                                    source_ml_source1_temporary_constructor_field948.clone(),
                                    Box::new(receiver.clone()),
                                    Some((source_ml_source2_constructor_field18393.clone(),)),
                                )
                            }
                        }
                    } else {
                        {
                            let source_ml_source2_constructor_field18407 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        Box::new(self::JavaScriptSemanticValue::Text(
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                            ])),
                                        )),
                                        self::create_java_script_closure_metadata_descriptor(
                                            self::JavaScriptSemanticValue::Number(
                                                shared_constructor_field3,
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
                                            shared_constructor_field2.clone(),
                                        ),
                                    ),
                                ]));
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                identity.clone(),
                                Box::new(source_ml_source1_temporary_constructor_field943.clone()),
                                true,
                                true,
                                source_ml_source2_constructor_field18407.clone(),
                                owner.clone(),
                                ordinal,
                                source_ml_source1_temporary_constructor_field948.clone(),
                                Box::new(receiver.clone()),
                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_call_closure(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    owner: self::ml::Big,
    ordinal: f64,
    captures: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    receiver: self::JavaScriptSemanticValue,
    new_target: Option<self::JavaScriptSemanticValue>,
) -> self::JavaScriptClosureAllocation {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let directory = self::read_java_script_call_template_directory_error(
                        program.clone(),
                        heap.clone(),
                    );
                    if (directory != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                            Box::new(heap.clone()),
                            directory.clone(),
                        )
                    } else {
                        {
                            let template = {
                                let source_ml_source2_array_read_value18409 =
                                    (match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value18408,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value18408
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
                                    let source_ml_source2_array_read_index18410 =
                                        self::find_java_script_call_template(
                                            program.clone(),
                                            owner.clone(),
                                            ordinal,
                                        );
                                    {
                                        let ml_reference_array935 =
                                            source_ml_source2_array_read_value18409.clone();
                                        let ml_reference_position935 =
                                            source_ml_source2_array_read_index18410;
                                        let ml_reference_projection935 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array935.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position935,
                                                ),
                                            );
                                        ml_reference_projection935
                                    }
                                }
                            };
                            if (match (template.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || (({
                                let ml_reference_array936 = captures.clone();
                                let ml_reference_projection936 =
                                    ml_reference_array936.borrow().len() as f64;
                                ml_reference_projection936
                            }) != (match template.clone().expect("a proven present value")
                            {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value18412,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value18412,
                            }))) {
                                self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108, 111,
                                        99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105,
                                        114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                        108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 97,
                                        110, 100, 32, 101, 120, 97, 99, 116, 32, 99, 97, 112, 116,
                                        117, 114, 101, 32, 118, 101, 99, 116, 111, 114,
                                    ]),
                                )
                            } else {
                                if ((((identity <= (self::ml::Big::from_i128(0)))
                                    || (identity
                                        > (self::ml::Big::from_i128(18446744073709551615))))
                                    || ((self::find_java_script_ordinary_object(
                                        shared_constructor_field1.clone(),
                                        identity.clone(),
                                    )) >= (0f64)))
                                    || self::test_java_script_ordinary_symbol(
                                        shared_constructor_field2.clone(),
                                        identity.clone(),
                                    ))
                                {
                                    self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108,
                                            111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97,
                                            105, 110, 115, 32, 97, 32, 103, 101, 110, 117, 105,
                                            110, 101, 108, 121, 32, 102, 114, 101, 115, 104, 32,
                                            101, 120, 116, 101, 114, 110, 97, 108, 32, 103, 108,
                                            111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105,
                                            116, 121,
                                        ]),
                                    )
                                } else {
                                    {
                                        let capture_count = {
                                            let ml_reference_array937 = captures.clone();
                                            let ml_reference_projection937 =
                                                ml_reference_array937.borrow().len() as f64;
                                            ml_reference_projection937
                                        };
                                        {
                                            let properties = 2f64;
                                            {
                                                let source_ml_source1_temporary_values950 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source1_temporary_index950 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s18417 = self::source_ml_source1_temporary_allocate_java_script_call_closure_loop953(capture_count, properties, source_ml_source1_temporary_values950.clone(), source_ml_source1_temporary_index950.clone());
                                                        match source_ml_source2_s18417.clone() {
                                                                self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Result::MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Done(capture_count_2, properties_2) => {
                                                                    if (((({ let ml_reference_array938 = shared_constructor_field1.clone(); let ml_reference_projection938 = ml_reference_array938.borrow().len() as f64; ml_reference_projection938 }) >= (4096f64)) || (capture_count_2 > (1048576f64))) || (properties_2 > (16384f64))) {
                                                                        self::JavaScriptClosureAllocation::ClosureAllocationResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 44, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 99, 97, 112, 116, 117, 114, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_temporary_values951 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![receiver.clone()], { let ml_reference_value939 = captures.borrow().clone(); ml_reference_value939 })));
                                                                            {
                                                                                let source_ml_source1_temporary_index951 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source2_s18420 = self::source_ml_source1_temporary_allocate_java_script_call_closure_loop954(heap.clone(), source_ml_source1_temporary_values951.clone(), source_ml_source1_temporary_index951.clone());
                                                                                    match source_ml_source2_s18420.clone() {
                                                                                        self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Done => {
                                                                                            {
                                                                                                let closure = self::create_java_script_call_closure_object(template.clone().expect("a proven present value"), identity.clone(), owner.clone(), ordinal, captures.clone(), receiver.clone(), new_target.clone());
                                                                                                {
                                                                                                    let at = 0f64;
                                                                                                    {
                                                                                                        let at_2 = self::source_ml_source1_temporary_allocate_java_script_call_closure_loop955(heap.clone(), identity.clone(), at);
                                                                                                        {
                                                                                                            let next = {
                                                                                                                let source_ml_source2_nominal_field18429 = shared_constructor_field1.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field18431 = shared_constructor_field2.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field18433 = shared_constructor_field3.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field18435 = shared_constructor_field4.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field18437 = shared_constructor_field5.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field18439 = shared_constructor_field6.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override18441 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                            let source_ml_source2_concat_value18425 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value940 = shared_constructor_field1.borrow().clone(); ml_reference_value940 }, 0f64, at_2)));
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value18426 = std::rc::Rc::new(std::cell::RefCell::new(vec![closure.clone()]));
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value18427 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value941 = shared_constructor_field1.borrow().clone(); ml_reference_value941 }, at_2, f64::INFINITY)));
                                                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value942 = source_ml_source2_concat_value18425.borrow().clone(); ml_reference_value942 }), { let ml_reference_value943 = source_ml_source2_concat_value18426.borrow().clone(); ml_reference_value943 }), { let ml_reference_value944 = source_ml_source2_concat_value18427.borrow().clone(); ml_reference_value944 })
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }));
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_override18441.clone(), source_ml_source2_nominal_field18431.clone(), source_ml_source2_nominal_field18433.clone(), source_ml_source2_nominal_field18435.clone(), source_ml_source2_nominal_field18437.clone(), source_ml_source2_nominal_field18439.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let invalid = self::read_java_script_ordinary_heap_error(next.clone());
                                                                                                                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                    self::JavaScriptClosureAllocation::ClosureAllocationInvalid(Box::new(heap.clone()), invalid.clone())
                                                                                                                } else {
                                                                                                                    self::JavaScriptClosureAllocation::ClosureAllocationNormal(Box::new(next.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), true)))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Return(source_ml_source1_temporary_result955) => {
                                                                                            let source_ml_source1_temporary_result955 = *source_ml_source1_temporary_result955;
                                                                                            source_ml_source1_temporary_result955.clone()
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function5(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-((-value).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_signed_integer(value: f64) -> f64 {
    {
        let unsigned = self::dependency_function5(value);
        if (unsigned >= (2147483648f64)) {
            (unsigned - 4294967296f64)
        } else {
            unsigned
        }
    }
}
