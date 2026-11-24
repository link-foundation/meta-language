// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptClosureAllocation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result;
use super::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop941;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_template_directory_error_loop938(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut owner: self::ml::Big,
    mut ordinal: f64,
    mut instructions: f64,
    mut captures: f64,
    mut source_ml_source1_temporary_values933: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>,
    >,
    mut source_ml_source1_temporary_index933: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result {
    loop {
        return match program.clone() {
            self::JavaScriptCallProgram::CallProgram(
                shared_constructor_field11,
                shared_constructor_field12,
                _,
                _,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index933
                    < ({
                        let ml_reference_array2840 = source_ml_source1_temporary_values933.clone();
                        let ml_reference_projection2840 =
                            self::ml::Big::from_u128(ml_reference_array2840.borrow().len() as u128);
                        ml_reference_projection2840
                    }))
                {
                    {
                        let template = {
                            let ml_reference_array2841 =
                                source_ml_source1_temporary_values933.clone();
                            let ml_reference_position2841 =
                                source_ml_source1_temporary_index933.clone();
                            let ml_reference_projection2841 = self::ml_array::at(
                                &*ml_reference_array2841.borrow(),
                                ml_reference_position2841.to_index(),
                            );
                            ml_reference_projection2841
                        };
                        match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
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
                            ) => {
                                let shared_constructor_field8 = *shared_constructor_field8;
                                let shared_constructor_field9 = *shared_constructor_field9;
                                if (({
                                    let source_ml_source2_comparison_operand41046 = match template
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
                                            source_ml_source2_data_property_value41045,
                                            _,
                                        ) => ((source_ml_source2_data_property_value41045
                                            .clone()
                                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                        .0)
                                            .clone(),
                                    };
                                    {
                                        let source_ml_source2_comparison_operand41047 = None::<()>;
                                        !(match (
                                            source_ml_source2_comparison_operand41046.clone(),
                                            source_ml_source2_comparison_operand41047.clone(),
                                        ) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        })
                                    }
                                } && (((match template.clone() {
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
                                        source_ml_source2_data_property_value41049,
                                        _,
                                    ) => ((source_ml_source2_data_property_value41049
                                        .clone()
                                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"))
                                    != (self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 99, 116,
                                    ]))))
                                    && (((match template.clone() {
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
                                            source_ml_source2_data_property_value41051,
                                            _,
                                        ) => ((source_ml_source2_data_property_value41051
                                            .clone()
                                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                        != (self::ml_string::Text::from_units(vec![
                                            108, 101, 120, 105, 99, 97, 108,
                                        ]))))
                                {
                                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 114, 114, 111, 119, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                } else {
                                    if ((((((match template.clone() {
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
                                        let source_ml_source2_type_query_operand41067 =
                                            shared_constructor_field1.clone();
                                        self::ml_string::Text::from_units(vec![
                                            98, 105, 103, 105, 110, 116,
                                        ])
                                    }) != (self::ml_string::Text::from_units(
                                        vec![98, 105, 103, 105, 110, 116],
                                    )))) || (shared_constructor_field1
                                        <= (self::ml::Big::from_i128(0))))
                                        || (shared_constructor_field1
                                            > (self::ml::Big::from_i128(18446744073709551615))))
                                        || !(self::test_java_script_call_code_coordinate(
                                            shared_constructor_field2,
                                            4096f64,
                                        )))
                                        || (shared_constructor_field1 < owner))
                                        || ((shared_constructor_field1 == owner)
                                            && (shared_constructor_field2 <= ordinal)))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 32, 111, 114, 100, 105, 110, 97, 108, 115]))
                                    } else {
                                        {
                                            let owner_2 = shared_constructor_field1.clone();
                                            {
                                                let ordinal_2 = shared_constructor_field2;
                                                {
                                                    let fn_ = {
                                                        let source_ml_source2_array_read_value41082 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_array_read_index41083 = self::find_java_script_call_code_function(program.clone(), owner_2.clone());
                                                            {
                                                                let ml_reference_array2842 = source_ml_source2_array_read_value41082.clone();
                                                                let ml_reference_position2842 = source_ml_source2_array_read_index41083;
                                                                let ml_reference_projection2842 = self::ml_array::optional_at(&*ml_reference_array2842.borrow(), self::ml_array::number_index(ml_reference_position2842));
                                                                ml_reference_projection2842
                                                            }
                                                        }
                                                    };
                                                    if ((match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fn_.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value41086, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value41086.clone()
                                                        }
                                                    }) != shared_constructor_field3)) || !(self::ml_array::includes(&{ let ml_reference_value2843 = shared_constructor_field12.borrow().clone(); ml_reference_value2843 }, &shared_constructor_field3.clone(), 0f64, false, |a, b| a == b))) {
                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![97, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 100, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                    } else {
                                                        if (((((!(self::test_java_script_call_code_coordinate(shared_constructor_field6, 4097f64)) || !(self::test_java_script_call_code_coordinate(shared_constructor_field4, 4097f64))) || !(self::test_java_script_call_code_coordinate(shared_constructor_field5, 4097f64))) || (shared_constructor_field5 == (0f64))) || ((((shared_constructor_field4 + shared_constructor_field6) + 1f64)) > shared_constructor_field5)) || !(self::test_java_script_call_code_coordinate(shared_constructor_field10, (shared_constructor_field4 + 1f64)))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 44, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 110, 101, 101, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                                        } else {
                                                            {
                                                                let captures_2 = (captures + shared_constructor_field6);
                                                                {
                                                                    let instructions_2 = (instructions + { let ml_reference_array2844 = shared_constructor_field7.clone(); let ml_reference_projection2844 = ml_reference_array2844.borrow().len() as f64; ml_reference_projection2844 });
                                                                    if (((captures_2 > (1048576f64)) || (instructions_2 > (65536f64))) || (({ let ml_reference_array2845 = shared_constructor_field7.clone(); let ml_reference_projection2845 = ml_reference_array2845.borrow().len() as f64; ml_reference_projection2845 }) == (0f64))) {
                                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 99, 111, 100, 101, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 97, 116, 105, 99, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_temporary_values934 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source1_temporary_index934 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source2_s41109 = self::source_ml_source1_temporary_read_java_script_call_template_directory_error_loop941(template.clone(), source_ml_source1_temporary_values934.clone(), source_ml_source1_temporary_index934.clone());
                                                                                    match source_ml_source2_s41109.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Done => {
                                                                                            {
                                                                                                let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field8.clone());
                                                                                                if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(prototype_error.clone())
                                                                                                } else {
                                                                                                    if ({
                                                                                                        let source_ml_source2_s41112 = shared_constructor_field8.clone();
                                                                                                        match source_ml_source2_s41112.clone() {
                                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                false
                                                                                                            }
                                                                                                            _ => {
                                                                                                                true
                                                                                                            }
                                                                                                        }
                                                                                                    } || !(((match shared_constructor_field8.clone() {
                                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value41117) => {
                                                                                                            Some((source_ml_source2_data_property_value41117,))
                                                                                                        }
                                                                                                        _ => {
                                                                                                            None::<(bool,)>
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value")).0)) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let name_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field9.clone());
                                                                                                            if (name_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(name_error.clone())
                                                                                                            } else {
                                                                                                                if (({
                                                                                                                    let source_ml_source2_s41120 = shared_constructor_field9.clone();
                                                                                                                    match source_ml_source2_s41120.clone() {
                                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                    }
                                                                                                                } && {
                                                                                                                    let source_ml_source2_s41124 = shared_constructor_field9.clone();
                                                                                                                    match source_ml_source2_s41124.clone() {
                                                                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                    }
                                                                                                                }) || ((self::read_java_script_semantic_text_length(shared_constructor_field9.clone())) > (65536f64))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101]))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let source_ml_source1_temporary_index933_2 = source_ml_source1_temporary_index933.add(&self::ml::Big::from_i128(1));
                                                                                                                        {
                                                                                                                            (owner, ordinal, instructions, captures, source_ml_source1_temporary_index933) = (owner_2.clone(), ordinal_2, instructions_2, captures_2, source_ml_source1_temporary_index933_2.clone());
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
                                                                                        self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop941Return(source_ml_source1_temporary_result941) => {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Return(source_ml_source1_temporary_result941.clone())
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
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Result::MlSource1TemporaryReadJavaScriptCallTemplateDirectoryErrorLoop938Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_call_closure_loop955(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
) -> f64 {
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
                if ((at
                    < ({
                        let ml_reference_array2846 = shared_constructor_field1.clone();
                        let ml_reference_projection2846 =
                            ml_reference_array2846.borrow().len() as f64;
                        ml_reference_projection2846
                    }))
                    && ((({
                        let source_ml_source2_optional_chain_receiver41152 = ({
                            let source_ml_source2_array_read_value41150 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2847 =
                                    source_ml_source2_array_read_value41150.clone();
                                let ml_reference_position2847 = at;
                                let ml_reference_projection2847 = self::ml_array::optional_at(
                                    &*ml_reference_array2847.borrow(),
                                    self::ml_array::number_index(ml_reference_position2847),
                                );
                                ml_reference_projection2847
                            }
                        },);
                        if match (source_ml_source2_optional_chain_receiver41152.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain952 =
                                    ((source_ml_source2_optional_chain_receiver41152.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain952.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        source_ml_source2_data_property_value41153,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41153.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        source_ml_source2_data_property_value41154,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41154.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        source_ml_source2_data_property_value41155,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41155.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        source_ml_source2_data_property_value41156,
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
                                    ) => source_ml_source2_data_property_value41156.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        source_ml_source2_data_property_value41157,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41157.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        source_ml_source2_data_property_value41158,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41158.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        source_ml_source2_data_property_value41159,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41159.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        source_ml_source2_data_property_value41160,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41160.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source2_data_property_value41161,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41161.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        source_ml_source2_data_property_value41162,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41162.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        source_ml_source2_data_property_value41163,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41163.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        source_ml_source2_data_property_value41164,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41164.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        source_ml_source2_data_property_value41165,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41165.clone(),
                                }
                            })
                        } else {
                            None::<self::ml::Big>
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| identity.clone()))
                        < identity))
                {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    at
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result {
    MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Done,
    MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Return(
        Box<self::JavaScriptClosureAllocation>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_call_closure_loop954(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values951: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index951: self::ml::Big,
) -> self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result {
    loop {
        return if (source_ml_source1_temporary_index951
            < ({
                let ml_reference_array2848 = source_ml_source1_temporary_values951.clone();
                let ml_reference_projection2848 =
                    self::ml::Big::from_u128(ml_reference_array2848.borrow().len() as u128);
                ml_reference_projection2848
            })) {
            {
                let value = {
                    let ml_reference_array2849 = source_ml_source1_temporary_values951.clone();
                    let ml_reference_position2849 = source_ml_source1_temporary_index951.clone();
                    let ml_reference_projection2849 = self::ml_array::at(
                        &*ml_reference_array2849.borrow(),
                        ml_reference_position2849.to_index(),
                    );
                    ml_reference_projection2849
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Return(Box::new(self::JavaScriptClosureAllocation::ClosureAllocationInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index951_2 =
                                source_ml_source1_temporary_index951
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index951 =
                                    source_ml_source1_temporary_index951_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Result::MlSource1TemporaryAllocateJavaScriptCallClosureLoop954Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Result {
    MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_call_closure_loop953(
    mut capture_count: f64,
    mut properties: f64,
    mut source_ml_source1_temporary_values950: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index950: self::ml::Big,
) -> self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Result {
    loop {
        return if (source_ml_source1_temporary_index950
            < ({
                let ml_reference_array2850 = source_ml_source1_temporary_values950.clone();
                let ml_reference_projection2850 =
                    self::ml::Big::from_u128(ml_reference_array2850.borrow().len() as u128);
                ml_reference_projection2850
            })) {
            {
                let object = {
                    let ml_reference_array2851 = source_ml_source1_temporary_values950.clone();
                    let ml_reference_position2851 = source_ml_source1_temporary_index950.clone();
                    let ml_reference_projection2851 = self::ml_array::at(
                        &*ml_reference_array2851.borrow(),
                        ml_reference_position2851.to_index(),
                    );
                    ml_reference_projection2851
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2852 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41170,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41170.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41171,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41171.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41172,
                            ) => source_ml_source2_data_property_value41172.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41173,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41173.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41174,
                                _,
                            ) => source_ml_source2_data_property_value41174.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41175,
                                _,
                            ) => source_ml_source2_data_property_value41175.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41176,
                                _,
                            ) => source_ml_source2_data_property_value41176.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41177,
                                _,
                            ) => source_ml_source2_data_property_value41177.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41178,
                            ) => source_ml_source2_data_property_value41178.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41179,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41179.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41180,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41180.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41181,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41181.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41182,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41182.clone(),
                        };
                        let ml_reference_projection2852 =
                            ml_reference_array2852.borrow().len() as f64;
                        ml_reference_projection2852
                    });
                    match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            captures,
                            _,
                            _,
                        ) => {
                            let capture_count_2 = (capture_count + {
                                let ml_reference_array2853 = captures.clone();
                                let ml_reference_projection2853 =
                                    ml_reference_array2853.borrow().len() as f64;
                                ml_reference_projection2853
                            });
                            {
                                let source_ml_source1_temporary_index950_2 =
                                    source_ml_source1_temporary_index950
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (
                                        capture_count,
                                        properties,
                                        source_ml_source1_temporary_index950,
                                    ) = (
                                        capture_count_2,
                                        properties_2,
                                        source_ml_source1_temporary_index950_2.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                        _ => {
                            let source_ml_source1_temporary_index950_3 =
                                source_ml_source1_temporary_index950
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_source1_temporary_index950) =
                                    (properties_2, source_ml_source1_temporary_index950_3.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Result::MlSource1TemporaryAllocateJavaScriptCallClosureLoop953Done(capture_count, properties)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop956(
    mut count: f64,
    mut factor: f64,
    mut position: f64,
) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop957(
    mut operation: f64,
    mut first: f64,
    mut second: f64,
    mut result: f64,
    mut weight: f64,
    mut position: f64,
) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64)))
                            && (second_bit == (1f64)))
                            || ((operation == (1f64))
                                && ((first_bit == (1f64)) || (second_bit == (1f64)))))
                            || ((operation == (2f64)) && (first_bit != second_bit)))
                        {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) =
                                                (first_2, second_2, result_3, weight_2, position_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}
