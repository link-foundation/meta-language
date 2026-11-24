pub fn source_ml_source1_temporary_find_java_script_closed_class_field_end_loop6(mut values: Vec<self::ml_string::Text>, mut last: f64, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result {
    loop {
        return if (at < last) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (value == (self::ml_string::Text::from_units(vec![59]))) {
                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(at)
                } else {
                    if ((((((((value == (self::ml_string::Text::from_units(vec![44]))) || (value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) || (value == (self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114])))) || (value == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) || (value == (self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116])))) || (value == (self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100])))) || (value == (self::ml_string::Text::from_units(vec![61, 62])))) || (value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) {
                        self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return((-1f64))
                    } else {
                        if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                            {
                                let end = self::find_java_script_class_group_end(values.clone(), at);
                                if ((end < (0f64)) || (end >= last)) {
                                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return((-1f64))
                                } else {
                                    {
                                        let inner = (at + 1f64);
                                        {
                                            let source_ml_source2_s140 = self::source_ml_source1_temporary_find_java_script_closed_class_field_end_loop7(values.clone(), end, inner);
                                            match source_ml_source2_s140.clone() {
                                                self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Done => {
                                                    {
                                                        let at_2 = end;
                                                        {
                                                            let at_3 = (at_2 + 1f64);
                                                            {
                                                                at = at_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Return(source_ml_source1_temporary_result7) => {
                                                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(source_ml_source1_temporary_result7)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let at_4 = (at + 1f64);
                                {
                                    at = at_4;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result {
    MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(self::ml_string::Text, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9 {
    MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(self::ml_string::Text, f64),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_field_documentation_loop8(mut comment: self::ml_string::Text, mut result: self::ml_string::Text, mut previous: f64, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result {
    loop {
        return if (at >= (0f64)) {
            {
                let before = if (at == (0f64)) {
                    self::ml_string::Text::from_units(vec![32])
                } else {
                    {
                        let source_ml_source2_string_unit_operand141 = comment.clone();
                        {
                            let source_ml_source2_string_unit_operand142 = (at - 1f64);
                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if ((index < (0f64)) || (index >= (text.length()))) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let source_ml_source2_string_range_operand143 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand144 = index;
                                            {
                                                let source_ml_source2_string_range_operand145 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position146 = source_ml_source2_string_range_operand144;
                                                    {
                                                        let source_ml_source2_string_range_position147 = source_ml_source2_string_range_operand145;
                                                        source_ml_source2_string_range_operand143.slice(source_ml_source2_string_range_position146, source_ml_source2_string_range_position147)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source2_string_unit_operand141.clone(), source_ml_source2_string_unit_operand142)
                        }
                    }
                };
                {
                    let after = {
                        let source_ml_source2_string_unit_operand148 = comment.clone();
                        {
                            let source_ml_source2_string_unit_operand149 = (at + 5f64);
                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if ((index < (0f64)) || (index >= (text.length()))) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let source_ml_source2_string_range_operand150 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand151 = index;
                                            {
                                                let source_ml_source2_string_range_operand152 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position153 = source_ml_source2_string_range_operand151;
                                                    {
                                                        let source_ml_source2_string_range_position154 = source_ml_source2_string_range_operand152;
                                                        source_ml_source2_string_range_operand150.slice(source_ml_source2_string_range_position153, source_ml_source2_string_range_position154)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source2_string_unit_operand148.clone(), source_ml_source2_string_unit_operand149)
                        }
                    };
                    {
                        let source_ml_source2_s160 = if (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![32]), self::ml_string::Text::from_units(vec![9]), self::ml_string::Text::from_units(vec![13]), self::ml_string::Text::from_units(vec![10]), self::ml_string::Text::from_units(vec![42])], &before.clone(), 0f64, false, |a, b| a == b) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![32]), self::ml_string::Text::from_units(vec![9]), self::ml_string::Text::from_units(vec![13]), self::ml_string::Text::from_units(vec![10]), self::ml_string::Text::from_units(vec![123]), self::ml_string::Text::from_units(vec![])], &after.clone(), 0f64, false, |a, b| a == b)) {
                            {
                                let result_2 = result.concat(&({
                                    let source_ml_source2_string_range_operand161 = comment.clone();
                                    {
                                        let source_ml_source2_string_range_operand162 = previous;
                                        {
                                            let source_ml_source2_string_range_operand163 = at;
                                            {
                                                let source_ml_source2_string_range_position164 = source_ml_source2_string_range_operand162;
                                                {
                                                    let source_ml_source2_string_range_position165 = source_ml_source2_string_range_operand163;
                                                    source_ml_source2_string_range_operand161.slice(source_ml_source2_string_range_position164, source_ml_source2_string_range_position165)
                                                }
                                            }
                                        }
                                    }
                                }).concat(&self::ml_string::Text::from_units(vec![64, 114, 101, 116, 117, 114, 110, 115])));
                                {
                                    let previous_2 = (at + 5f64);
                                    self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_2.clone(), previous_2)
                                }
                            }
                        } else {
                            self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result.clone(), previous)
                        };
                        match source_ml_source2_s160.clone() {
                            self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_3, previous_3) => {
                                {
                                    let at_2 = {
                                        let source_ml_source2_string_search_operand166 = comment.clone();
                                        {
                                            let source_ml_source2_string_search_operand167 = self::ml_string::Text::from_units(vec![64, 116, 121, 112, 101]);
                                            {
                                                let source_ml_source2_string_search_operand168 = (at + 5f64);
                                                {
                                                    let source_ml_source2_string_search_text169 = source_ml_source2_string_search_operand167.clone();
                                                    {
                                                        let source_ml_source2_string_search_position170 = source_ml_source2_string_search_operand168;
                                                        source_ml_source2_string_search_operand166.index_of(&source_ml_source2_string_search_text169, source_ml_source2_string_search_position170)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        (result, previous, at) = (result_3.clone(), previous_3, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result::MlSource1TemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(result.clone(), previous)
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_prefix_end_loop10(mut row: Vec<f64>, mut end: f64, mut fields: f64, mut at: f64) -> f64 {
    loop {
        return if (at < ((9f64 + (fields * 3f64)))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let end_2 = ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64) + 1f64);
                    {
                        let at_2 = (at + 3f64);
                        {
                            (end, at) = (end_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 3f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result {
    MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done,
    MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_assigned_field_position_loop11(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut name: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result {
    loop {
        return if (at < ((9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64))))) {
            if ((((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == name)) {
                self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(at)
            } else {
                {
                    let at_2 = (at + 3f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlSource1TemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14 {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Done,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop15(mut values: Vec<self::ml_string::Text>, mut body_end: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result {
    loop {
        return if (at < body_end) {
            if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Return(vec![(-6f64), at])
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop16(mut values: Vec<self::ml_string::Text>, mut fields: Vec<f64>, mut name: self::ml_string::Text, mut declaration: bool, mut field: f64) -> bool {
    loop {
        return if (field < ((fields.len() as f64))) {
            if ((((self::ml_array::optional_at(&fields, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64)) < (0f64)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(((-((self::ml_array::optional_at(&fields, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64))) - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == name)) {
                {
                    let declaration_2 = true;
                    {
                        let field_2 = (field + 3f64);
                        {
                            (declaration, field) = (declaration_2, field_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let field_3 = (field + 3f64);
                    {
                        field = field_3;
                        continue;
                    }
                }
            }
        } else {
            declaration
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(f64),
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Done,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop19(mut values: Vec<self::ml_string::Text>, mut group: f64, mut inner: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result {
    loop {
        return if (inner < group) {
            if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(inner))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Return(vec![(-5f64), inner])
            } else {
                {
                    let inner_2 = (inner + 1f64);
                    {
                        inner = inner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop18(mut values: Vec<self::ml_string::Text>, mut body_end: f64, mut finish: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result {
    loop {
        return if (finish < body_end) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(finish))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (value == (self::ml_string::Text::from_units(vec![59]))) {
                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish)
                } else {
                    if (value == (self::ml_string::Text::from_units(vec![44]))) {
                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![(-5f64), finish])
                    } else {
                        if (((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((finish - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((finish - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) {
                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![(-5f64), finish])
                        } else {
                            if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                {
                                    let group = self::find_java_script_class_group_end(values.clone(), finish);
                                    if ((group < (0f64)) || (group >= body_end)) {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![(-5f64), finish])
                                    } else {
                                        {
                                            let inner = (finish + 1f64);
                                            {
                                                let source_ml_source2_s171 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop19(values.clone(), group, inner);
                                                match source_ml_source2_s171.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Done => {
                                                        {
                                                            let finish_2 = group;
                                                            {
                                                                let finish_3 = (finish_2 + 1f64);
                                                                {
                                                                    finish = finish_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop19Return(source_ml_source1_temporary_result19) => {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(source_ml_source1_temporary_result19)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let finish_4 = (finish + 1f64);
                                    {
                                        finish = finish_4;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Done,
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop17(mut values: Vec<self::ml_string::Text>, mut fields: Vec<f64>, mut cursor: f64, mut name: self::ml_string::Text, mut previous: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result {
    loop {
        return if (previous < ((fields.len() as f64))) {
            if ((((self::ml_array::optional_at(&fields, self::ml_array::number_index(previous))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&fields, self::ml_array::number_index(previous))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == name)) {
                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Return(vec![(-4f64), (cursor + 2f64)])
            } else {
                {
                    let previous_2 = (previous + 3f64);
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop13(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut fields: Vec<f64>, mut names: Vec<self::ml_string::Text>, mut body_start: f64, mut body_end: f64, mut cursor: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result {
    loop {
        return if ((((((body_start >= (0f64)) && (cursor < body_end)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(cursor))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((cursor + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((cursor + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((cursor + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61])))) {
            {
                let name = (self::ml_array::optional_at(&values, self::ml_array::number_index((cursor + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let declaration = false;
                    {
                        let field = 0f64;
                        {
                            let declaration_2 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop16(values.clone(), fields.clone(), name.clone(), declaration, field);
                            if ((name == (self::ml_string::Text::from_units(vec![36]))) || (self::ml_array::includes(&names, &(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name), 0f64, false, |a, b| a == b) && !declaration_2)) {
                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(vec![(-4f64), (cursor + 2f64)])
                            } else {
                                {
                                    let previous = 0f64;
                                    {
                                        let source_ml_source2_s172 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop17(values.clone(), fields.clone(), cursor, name.clone(), previous);
                                        match source_ml_source2_s172.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Done => {
                                                {
                                                    let start = (cursor + 4f64);
                                                    {
                                                        let finish = start;
                                                        {
                                                            let source_ml_source2_s173 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop18(values.clone(), body_end, finish);
                                                            match source_ml_source2_s173.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish_2) => {
                                                                    if ((finish_2 == start) || (finish_2 >= body_end)) {
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(vec![(-5f64), start])
                                                                    } else {
                                                                        {
                                                                            let fields_2 = {
                                                                                let source_ml_source2_concat_value174 = fields;
                                                                                {
                                                                                    let source_ml_source2_concat_value175 = vec![(cursor + 2f64), start, finish_2];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value174), source_ml_source2_concat_value175)
                                                                                }
                                                                            };
                                                                            {
                                                                                let cursor_2 = (finish_2 + 1f64);
                                                                                {
                                                                                    (fields, cursor) = (fields_2, cursor_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop18Return(source_ml_source1_temporary_result18) => {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_source1_temporary_result18)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop17Return(source_ml_source1_temporary_result19) => {
                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_source1_temporary_result19)
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
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Done(fields)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Done(Vec<f64>, Vec<f64>, Vec<f64>, Vec<self::ml_string::Text>),
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20 {
    MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(Vec<f64>, Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_directory_loop12(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut end: f64, mut constructor: Vec<f64>, mut methods: Vec<f64>, mut fields: Vec<f64>, mut names: Vec<self::ml_string::Text>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    loop {
        return if (at < end) {
            if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![59]))) {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            } else {
                {
                    let static_member = (((((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40]))));
                    {
                        let position = if static_member {
                            (at + 1f64)
                        } else {
                            at
                        };
                        {
                            let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let owner = (if static_member {
                                    self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99, 58])
                                } else {
                                    self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])
                                }).concat(&name);
                                {
                                    let field = ((((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![59]))));
                                    if ((!static_member && field) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) {
                                        if (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), self::ml_string::Text::from_units(vec![95, 95, 112, 114, 111, 116, 111, 95, 95]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), self::ml_string::Text::from_units(vec![36])], &name.clone(), 0f64, false, |a, b| a == b) || self::ml_array::includes(&names, &(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name), 0f64, false, |a, b| a == b)) {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), position])
                                        } else {
                                            {
                                                let initial = if (((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                                                    (position + 2f64)
                                                } else {
                                                    (position + 1f64)
                                                };
                                                {
                                                    let finish = self::find_java_script_closed_class_field_end(values.clone(), initial, end);
                                                    if ((finish < (0f64)) || ((((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) && (finish == initial))) {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-10f64), initial])
                                                    } else {
                                                        {
                                                            let fields_2 = {
                                                                let source_ml_source2_concat_value176 = fields;
                                                                {
                                                                    let source_ml_source2_concat_value177 = vec![((-position) - 1f64), initial, finish];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value176), source_ml_source2_concat_value177)
                                                                }
                                                            };
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source2_concat_value178 = names;
                                                                    {
                                                                        let source_ml_source2_concat_value179 = vec![(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name)];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value178), source_ml_source2_concat_value179)
                                                                    }
                                                                };
                                                                {
                                                                    let at_3 = (finish + 1f64);
                                                                    {
                                                                        (fields, names, at) = (fields_2, names_2, at_3);
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
                                        if (((((((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (name == (self::ml_string::Text::from_units(vec![95, 95, 112, 114, 111, 116, 111, 95, 95])))) || (name == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101])))) || (name == (self::ml_string::Text::from_units(vec![36])))) || (static_member && (name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))))) || self::ml_array::includes(&names, &owner.clone(), 0f64, false, |a, b| a == b)) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((position + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![40])))) {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), at])
                                        } else {
                                            {
                                                let parameters = self::find_java_script_class_group_end(values.clone(), (position + 1f64));
                                                if ((parameters < (0f64)) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((parameters + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![123])))) {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), at])
                                                } else {
                                                    {
                                                        let literal_positions = self::read_java_script_closed_class_parameter_positions(values.clone(), kinds.clone(), (position + 1f64), parameters);
                                                        {
                                                            let positions = if (((self::ml_array::optional_at(&literal_positions, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64)) < (0f64)) {
                                                                self::read_java_script_closed_class_collection_parameter_positions(values.clone(), kinds.clone(), (position + 1f64), parameters)
                                                            } else {
                                                                literal_positions
                                                            };
                                                            if (((self::ml_array::optional_at(&positions, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64)) < (0f64)) {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-3f64), (self::ml_array::optional_at(&positions, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| at)])
                                                            } else {
                                                                {
                                                                    let body = self::find_java_script_class_group_end(values.clone(), (parameters + 1f64));
                                                                    if ((body < (0f64)) || (body > end)) {
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![(-2f64), at])
                                                                    } else {
                                                                        {
                                                                            let names_3 = {
                                                                                let source_ml_source2_concat_value180 = names;
                                                                                {
                                                                                    let source_ml_source2_concat_value181 = vec![owner.clone()];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value180), source_ml_source2_concat_value181)
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_s184 = if ((name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && !static_member) {
                                                                                    {
                                                                                        let constructor_2 = vec![position, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor_2, methods)
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let methods_2 = {
                                                                                            let source_ml_source2_concat_value185 = methods;
                                                                                            {
                                                                                                let source_ml_source2_concat_value186 = vec![if static_member {
                                                                                                    ((-position) - 1f64)
                                                                                                } else {
                                                                                                    position
                                                                                                }, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value185), source_ml_source2_concat_value186)
                                                                                            }
                                                                                        };
                                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor, methods_2)
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s184.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor_3, methods_3) => {
                                                                                        {
                                                                                            let at_4 = (body + 1f64);
                                                                                            {
                                                                                                (constructor, methods, names, at) = (constructor_3, methods_3, names_3, at_4);
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
            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Done(constructor, methods, fields, names)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result {
    MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Done(self::ml_string::Text, f64),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_range_loop21(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut last: f64, mut receiver: self::ml_string::Text, mut class_names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut result: self::ml_string::Text, mut previous: f64, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result {
    loop {
        return if (at < last) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let target = self::read_java_script_closed_class_static_target(values.clone(), class_names.clone(), static_calls.clone(), at);
                    if (target != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let result_2 = result.concat(&({
                                let source_ml_source2_string_range_operand187 = source.clone();
                                {
                                    let source_ml_source2_string_range_operand188 = previous;
                                    {
                                        let source_ml_source2_string_range_operand189 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source2_string_range_position190 = source_ml_source2_string_range_operand188;
                                            {
                                                let source_ml_source2_string_range_position191 = source_ml_source2_string_range_operand189;
                                                source_ml_source2_string_range_operand187.slice(source_ml_source2_string_range_position190, source_ml_source2_string_range_position191)
                                            }
                                        }
                                    }
                                }
                            }).concat(&target));
                            {
                                let previous_2 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| previous);
                                {
                                    let at_2 = (at + 2f64);
                                    {
                                        let at_3 = (at_2 + 1f64);
                                        {
                                            (result, previous, at) = (result_2.clone(), previous_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let this_reference = ((((receiver != (self::ml_string::Text::from_units(vec![]))) && (value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46]))));
                            {
                                let construction = (((value == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&class_names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40]))));
                                if (this_reference || construction) {
                                    {
                                        let result_3 = result.concat(&({
                                            let source_ml_source2_string_range_operand192 = source.clone();
                                            {
                                                let source_ml_source2_string_range_operand193 = previous;
                                                {
                                                    let source_ml_source2_string_range_operand194 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| previous);
                                                    {
                                                        let source_ml_source2_string_range_position195 = source_ml_source2_string_range_operand193;
                                                        {
                                                            let source_ml_source2_string_range_position196 = source_ml_source2_string_range_operand194;
                                                            source_ml_source2_string_range_operand192.slice(source_ml_source2_string_range_position195, source_ml_source2_string_range_position196)
                                                        }
                                                    }
                                                }
                                            }
                                        }).concat(&if this_reference {
                                            receiver.clone()
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }));
                                        {
                                            let previous_3 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(at))).clone().unwrap_or_else(|| previous);
                                            {
                                                let at_4 = (at + 1f64);
                                                {
                                                    (result, previous, at) = (result_3.clone(), previous_3, at_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_5 = (at + 1f64);
                                        {
                                            at = at_5;
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
            self::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Done(result.clone(), previous)
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_methods_loop22(mut values: Vec<self::ml_string::Text>, mut directory: Vec<f64>, mut result: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((directory.len() as f64))) {
            if (((self::ml_array::optional_at(&directory, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value197 = result;
                        {
                            let source_ml_source2_concat_value198 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&directory, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value197), source_ml_source2_concat_value198)
                        }
                    };
                    {
                        let at_2 = (at + 5f64);
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 5f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop25(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut result: Vec<self::ml_string::Text>, mut ordinal: f64, mut row: Vec<f64>, mut name: self::ml_string::Text, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((row.len() as f64))) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                if (position < (0f64)) {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value201 = result;
                            {
                                let source_ml_source2_concat_value202 = vec![(name.concat(&self::ml_string::Text::from_units(vec![46]))).concat(&(self::ml_array::optional_at(&values, self::ml_array::number_index(((-position) - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                    let source_ml_source2_string_argument199 = ordinal;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument199))
                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                    let source_ml_source2_string_argument200 = at;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument200))
                                })];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value201), source_ml_source2_concat_value202)
                            }
                        };
                        {
                            let at_2 = (at + 5f64);
                            {
                                (result, at) = (result_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 5f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}
