pub fn source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop24(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut result: Vec<self::ml_string::Text>, mut ordinal: f64, mut source_ml_source1_temporary_values23: Vec<Vec<f64>>, mut source_ml_source1_temporary_index23: self::ml::Big) -> Vec<self::ml_string::Text> {
    loop {
        return if (source_ml_source1_temporary_index23 < (self::ml::Big::from_u128(source_ml_source1_temporary_values23.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values23, source_ml_source1_temporary_index23.to_index());
                {
                    let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let at = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                        {
                            let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop25(values.clone(), prefix.clone(), result, ordinal, row, name.clone(), at);
                            {
                                let ordinal_2 = (ordinal + 1f64);
                                {
                                    let source_ml_source1_temporary_index23_2 = source_ml_source1_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, ordinal, source_ml_source1_temporary_index23) = (result_2, ordinal_2, source_ml_source1_temporary_index23_2.clone());
                                        continue;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result {
    MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Done,
    MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_static_target_loop26(mut calls: Vec<self::ml_string::Text>, mut key: self::ml_string::Text, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result {
    loop {
        return if (index < ((calls.len() as f64))) {
            if (((self::ml_array::optional_at(&calls, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == key) {
                self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Return((self::ml_array::optional_at(&calls, self::ml_array::number_index((index + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            } else {
                {
                    let index_2 = (index + 2f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values27: Vec<Vec<f64>>, mut source_ml_source1_temporary_index27: self::ml::Big) -> Vec<self::ml_string::Text> {
    loop {
        return if (source_ml_source1_temporary_index27 < (self::ml::Big::from_u128(source_ml_source1_temporary_values27.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values27, source_ml_source1_temporary_index27.to_index());
                {
                    let names_2 = {
                        let source_ml_source2_concat_value203 = names;
                        {
                            let source_ml_source2_concat_value204 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value203), source_ml_source2_concat_value204)
                        }
                    };
                    {
                        let source_ml_source1_temporary_index27_2 = source_ml_source1_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_source1_temporary_index27) = (names_2, source_ml_source1_temporary_index27_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result {
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done,
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop31(mut row: Vec<f64>, mut parameters: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((row.len() as f64))) {
            {
                let parameters_2 = {
                    let source_ml_source2_concat_value205 = parameters;
                    {
                        let source_ml_source2_concat_value206 = vec![(self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64)), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| (-1f64))];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value205), source_ml_source2_concat_value206)
                    }
                };
                {
                    let at_2 = (at + 5f64);
                    {
                        (parameters, at) = (parameters_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            parameters
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result {
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done,
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result {
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done,
    MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop33(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut parameters: Vec<f64>, mut pair: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result {
    loop {
        return if (at < ((self::ml_array::optional_at(&parameters, self::ml_array::number_index((pair + 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            if ((((((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 101, 119])))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) {
                self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return((at + 1f64))
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
            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop32(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut parameters: Vec<f64>, mut pair: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result {
    loop {
        return if (pair < ((parameters.len() as f64))) {
            {
                let at = ((self::ml_array::optional_at(&parameters, self::ml_array::number_index(pair))).clone().unwrap_or_else(|| (-1f64)) + 1f64);
                {
                    let source_ml_source2_s207 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop33(values.clone(), names.clone(), parameters.clone(), pair, at);
                    match source_ml_source2_s207.clone() {
                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done => {
                            {
                                let pair_2 = (pair + 2f64);
                                {
                                    pair = pair_2;
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(source_ml_source1_temporary_result33) => {
                            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(source_ml_source1_temporary_result33)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values28: Vec<Vec<f64>>, mut source_ml_source1_temporary_index28: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result {
    loop {
        return if (source_ml_source1_temporary_index28 < (self::ml::Big::from_u128(source_ml_source1_temporary_values28.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values28, source_ml_source1_temporary_index28.to_index());
                {
                    let parameters = vec![(self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| (-1f64)), (self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| (-1f64))];
                    {
                        let at = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                        {
                            let parameters_2 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop31(row, parameters, at);
                            {
                                let pair = 0f64;
                                {
                                    let source_ml_source2_s208 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop32(values.clone(), names.clone(), parameters_2, pair);
                                    match source_ml_source2_s208.clone() {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done => {
                                            {
                                                let source_ml_source1_temporary_index28_2 = source_ml_source1_temporary_index28.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index28 = source_ml_source1_temporary_index28_2.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(source_ml_source1_temporary_result32) => {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(source_ml_source1_temporary_result32)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_function_exports_loop34(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut result: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((values.len() as f64))) {
            if ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::accept_java_script_closed_class_function_export(values.clone(), kinds.clone(), at)) {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value209 = result;
                        {
                            let source_ml_source2_concat_value210 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value209), source_ml_source2_concat_value210)
                        }
                    };
                    {
                        let at_2 = (at + 1f64);
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39 {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop40(mut at: f64, mut source_ml_source1_temporary_values36: Vec<Vec<f64>>, mut source_ml_source1_temporary_index36: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result {
    loop {
        return if (source_ml_source1_temporary_index36 < (self::ml::Big::from_u128(source_ml_source1_temporary_values36.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values36, source_ml_source1_temporary_index36.to_index());
                if ((at > ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64))) && (at < ((self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| 0f64)))) {
                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
                } else {
                    {
                        let source_ml_source1_temporary_index36_2 = source_ml_source1_temporary_index36.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index36 = source_ml_source1_temporary_index36_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop38(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut directory: Vec<Vec<f64>>, mut static_calls: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut methods: Vec<self::ml_string::Text>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let kind = (self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let next = (self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            if ((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![74, 83, 79, 78]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108]), self::ml_string::Text::from_units(vec![80, 114, 111, 120, 121]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 118, 97, 108]), self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]), self::ml_string::Text::from_units(vec![100, 101, 101, 112, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![110, 111, 116, 68, 101, 101, 112, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![100, 101, 101, 112, 83, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![110, 111, 116, 68, 101, 101, 112, 83, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108])], &value.clone(), 0f64, false, |a, b| a == b)) {
                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                            } else {
                                if (((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && (value == (self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116])))) && !(self::accept_java_script_closed_class_function_export(values.clone(), kinds.clone(), at))) {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                } else {
                                    if (((value == (self::ml_string::Text::from_units(vec![46, 46, 46]))) || (value == (self::ml_string::Text::from_units(vec![95, 95, 112, 114, 111, 116, 111, 95, 95])))) || (value == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101])))) {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                    } else {
                                        {
                                            let source_ml_source2_s212 = if (value == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                {
                                                    let source_ml_source1_temporary_values36 = directory.clone();
                                                    {
                                                        let source_ml_source1_temporary_index36 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s213 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop40(at, source_ml_source1_temporary_values36, source_ml_source1_temporary_index36.clone());
                                                            match source_ml_source2_s213.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Done => {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(source_ml_source1_temporary_result40) => {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(source_ml_source1_temporary_result40)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next
                                            };
                                            match source_ml_source2_s212.clone() {
                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Next => {
                                                    if (((((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(&names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40]))))) && ((self::read_java_script_closed_class_static_target(values.clone(), names.clone(), static_calls.clone(), at)) == (self::ml_string::Text::from_units(vec![])))) {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                    } else {
                                                        if (((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91])))) {
                                                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                        } else {
                                                            if (((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((self::ml_array::includes(&methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) || (value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))))) {
                                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                            } else {
                                                                if (((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(&methods, &{
                                                                    let source_ml_source2_string_range_operand214 = value.clone();
                                                                    {
                                                                        let source_ml_source2_string_range_operand215 = 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position216 = source_ml_source2_string_range_operand215;
                                                                            {
                                                                                let source_ml_source2_string_range_position217 = f64::INFINITY;
                                                                                source_ml_source2_string_range_operand214.slice(source_ml_source2_string_range_position216, source_ml_source2_string_range_position217)
                                                                            }
                                                                        }
                                                                    }
                                                                }, 0f64, false, |a, b| a == b)) {
                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                                } else {
                                                                    {
                                                                        let at_2 = (at + 1f64);
                                                                        {
                                                                            at = at_2;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(source_ml_source1_temporary_result39) => {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(source_ml_source1_temporary_result39)
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
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(Vec<self::ml_string::Text>, Vec<self::ml_string::Text>),
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Done,
    MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop42(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut at: f64, mut inner: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    loop {
        return if (inner < ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 4f64)))).clone().unwrap_or_else(|| 0f64))) {
            if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(inner))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((inner - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(inner)
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
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop41(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result {
    loop {
        return if (at < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) < (0f64)) {
                {
                    let inner = ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| 0f64) + 1f64);
                    {
                        let source_ml_source2_s218 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop42(values.clone(), row.clone(), at, inner);
                        match source_ml_source2_s218.clone() {
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Done => {
                                {
                                    let at_2 = (at + 5f64);
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(source_ml_source1_temporary_result42) => {
                                self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(source_ml_source1_temporary_result42)
                            }
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
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop37(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut methods: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values35: Vec<Vec<f64>>, mut source_ml_source1_temporary_index35: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result {
    loop {
        return if (source_ml_source1_temporary_index35 < (self::ml::Big::from_u128(source_ml_source1_temporary_values35.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.to_index());
                {
                    let names_2 = {
                        let source_ml_source2_concat_value219 = names;
                        {
                            let source_ml_source2_concat_value220 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value219), source_ml_source2_concat_value220)
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source2_concat_value221 = methods;
                            {
                                let source_ml_source2_concat_value222 = self::read_java_script_closed_class_methods(values.clone(), row.clone());
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value221), source_ml_source2_concat_value222)
                            }
                        };
                        {
                            let at = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                            {
                                let source_ml_source2_s223 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop41(values.clone(), row, at);
                                match source_ml_source2_s223.clone() {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Done => {
                                        {
                                            let source_ml_source1_temporary_index35_2 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                                            {
                                                (names, methods, source_ml_source1_temporary_index35) = (names_2, methods_2, source_ml_source1_temporary_index35_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(source_ml_source1_temporary_result41) => {
                                        self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(source_ml_source1_temporary_result41)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(names, methods)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result {
    MlSource1TemporaryRenderJavaScriptClosedClassLoop44Done(self::ml_string::Text, self::ml_string::Text, self::ml_string::Text),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_loop44(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut row: Vec<f64>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut class_names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut prototype: bool, mut receiver: self::ml_string::Text, mut stubs: self::ml_string::Text, mut fields: self::ml_string::Text, mut assignments: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result {
    loop {
        return if (at < ((row.len() as f64))) {
            {
                let member = (self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let position = if (member < (0f64)) {
                        ((-member) - 1f64)
                    } else {
                        member
                    };
                    {
                        let method = (self::ml_array::optional_at(&values, self::ml_array::number_index(position))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let args = self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64) + 1f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64), self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone());
                            {
                                let arguments_ = self::render_java_script_closed_class_parameter_arguments(source.clone(), values.clone(), starts.clone(), ends.clone(), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64));
                                {
                                    let body = self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| 0f64) + 1f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 4f64)))).clone().unwrap_or_else(|| 0f64), if (member < (0f64)) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        receiver.clone()
                                    }, class_names.clone(), static_calls.clone());
                                    {
                                        let stub = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 95]))).concat(&{
                                            let source_ml_source2_string_argument224 = ordinal;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument224))
                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                            let source_ml_source2_string_argument225 = at;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument225))
                                        });
                                        {
                                            let declaration = if (member < (0f64)) {
                                                (position - 1f64)
                                            } else {
                                                position
                                            };
                                            {
                                                let documentation = {
                                                    let source_ml_source2_string_range_operand226 = source.clone();
                                                    {
                                                        let source_ml_source2_string_range_operand227 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((declaration - 1f64)))).clone().unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source2_string_range_operand228 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(declaration))).clone().unwrap_or_else(|| 0f64);
                                                            {
                                                                let source_ml_source2_string_range_position229 = source_ml_source2_string_range_operand227;
                                                                {
                                                                    let source_ml_source2_string_range_position230 = source_ml_source2_string_range_operand228;
                                                                    source_ml_source2_string_range_operand226.slice(source_ml_source2_string_range_position229, source_ml_source2_string_range_position230)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                if (member < (0f64)) {
                                                    {
                                                        let stubs_2 = stubs.concat(&((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                                            let source_ml_source2_string_argument231 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument231))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source2_string_argument232 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument232))
                                                        })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                        {
                                                            let at_2 = (at + 5f64);
                                                            {
                                                                (stubs, at) = (stubs_2.clone(), at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if prototype {
                                                        {
                                                            let stubs_3 = stubs.concat(&(((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&{
                                                                let source_ml_source2_string_argument233 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument233))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source2_string_argument234 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument234))
                                                            })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&receiver)).concat(&if (args == (self::ml_string::Text::from_units(vec![]))) {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                (self::ml_string::Text::from_units(vec![44, 32])).concat(&args)
                                                            })).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                            {
                                                                let at_3 = (at + 5f64);
                                                                {
                                                                    (stubs, at) = (stubs_3.clone(), at_3);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let environment = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 95]))).concat(&{
                                                                let source_ml_source2_string_argument235 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument235))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source2_string_argument236 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument236))
                                                            });
                                                            {
                                                                let stubs_4 = stubs.concat(&((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&arguments_)).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 10])));
                                                                {
                                                                    let stubs_5 = stubs_4.concat(&((((((((self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32])).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32, 40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 61, 62, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 32, 125, 10])));
                                                                    {
                                                                        let fields_2 = fields.concat(&(((self::ml_string::Text::from_units(vec![44, 32])).concat(&method)).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&stub));
                                                                        {
                                                                            let assignments_2 = assignments.concat(&((((((((self::ml_string::Text::from_units(vec![10])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&method)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                                            {
                                                                                let at_4 = (at + 5f64);
                                                                                {
                                                                                    (stubs, fields, assignments, at) = (stubs_5.clone(), fields_2.clone(), assignments_2.clone(), at_4);
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
        } else {
            self::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result {
    MlSource1TemporaryRenderJavaScriptClosedClassLoop43Done(self::ml_string::Text, self::ml_string::Text, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassJoin46 {
    MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderJavaScriptClosedClassJoin45 {
    MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(self::ml_string::Text, self::ml_string::Text, self::ml_string::Text),
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_loop43(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut row: Vec<f64>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut class_names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut methods: f64, mut stubs: self::ml_string::Text, mut initializations: self::ml_string::Text, mut constructor_fields: self::ml_string::Text, mut field_names: Vec<self::ml_string::Text>, mut fields: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result {
    loop {
        return if (at < methods) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let field = (self::ml_array::optional_at(&values, self::ml_array::number_index(if (position < (0f64)) {
                        ((-position) - 1f64)
                    } else {
                        position
                    }))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let value = if (((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64)) == ((self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64))) {
                            self::ml_string::Text::from_units(vec![40, 118, 111, 105, 100, 32, 48, 41])
                        } else {
                            self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64), (self::ml_array::optional_at(&row, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64), self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone())
                        };
                        {
                            let source_ml_source2_s248 = if (position < (0f64)) {
                                {
                                    let initializer = (((prefix.concat(&self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 95, 99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95]))).concat(&{
                                        let source_ml_source2_string_argument249 = ordinal;
                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument249))
                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                        let source_ml_source2_string_argument250 = at;
                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument250))
                                    });
                                    {
                                        let capture = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                            let source_ml_source2_string_argument251 = ordinal;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument251))
                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                            let source_ml_source2_string_argument252 = at;
                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument252))
                                        });
                                        {
                                            let documentation = self::render_java_script_closed_class_field_documentation({
                                                let source_ml_source2_string_range_operand253 = source.clone();
                                                {
                                                    let source_ml_source2_string_range_operand254 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(((-position) - 2f64)))).clone().unwrap_or_else(|| 0f64);
                                                    {
                                                        let source_ml_source2_string_range_operand255 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(((-position) - 1f64)))).clone().unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source2_string_range_position256 = source_ml_source2_string_range_operand254;
                                                            {
                                                                let source_ml_source2_string_range_position257 = source_ml_source2_string_range_operand255;
                                                                source_ml_source2_string_range_operand253.slice(source_ml_source2_string_range_position256, source_ml_source2_string_range_position257)
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                            {
                                                let stubs_2 = stubs.concat(&((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&initializer)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 10])));
                                                {
                                                    let initializations_2 = initializations.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&initializer)).concat(&self::ml_string::Text::from_units(vec![40, 41, 59, 10])));
                                                    {
                                                        let value_2 = capture.clone();
                                                        {
                                                            let assigned = self::read_java_script_closed_class_assigned_field_position(values.clone(), row.clone(), field.clone());
                                                            if (assigned >= (0f64)) {
                                                                {
                                                                    let value_3 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                                        let source_ml_source2_string_argument258 = ordinal;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument258))
                                                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                        let source_ml_source2_string_argument259 = assigned;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument259))
                                                                    });
                                                                    self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs_2.clone(), initializations_2.clone(), value_3.clone())
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs_2.clone(), initializations_2.clone(), value_2.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs.clone(), initializations.clone(), value.clone())
                            };
                            match source_ml_source2_s248.clone() {
                                self::MlSource1TemporaryRenderJavaScriptClosedClassJoin45::MlSource1TemporaryRenderJavaScriptClosedClassJoin45Next(stubs_3, initializations_3, value_4) => {
                                    {
                                        let source_ml_source2_s262 = if (position >= (0f64)) {
                                            {
                                                let capture_2 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                    let source_ml_source2_string_argument263 = ordinal;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument263))
                                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                    let source_ml_source2_string_argument264 = at;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument264))
                                                });
                                                {
                                                    let constructor_fields_2 = constructor_fields.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture_2)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 40]))).concat(&value_4)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                    {
                                                        let value_5 = capture_2.clone();
                                                        self::MlSource1TemporaryRenderJavaScriptClosedClassJoin46::MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields_2.clone(), value_5.clone())
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryRenderJavaScriptClosedClassJoin46::MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields.clone(), value_4.clone())
                                        };
                                        match source_ml_source2_s262.clone() {
                                            self::MlSource1TemporaryRenderJavaScriptClosedClassJoin46::MlSource1TemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields_3, value_6) => {
                                                if self::ml_array::includes(&field_names, &field.clone(), 0f64, false, |a, b| a == b) {
                                                    {
                                                        let at_2 = (at + 3f64);
                                                        {
                                                            (stubs, initializations, constructor_fields, at) = (stubs_3.clone(), initializations_3.clone(), constructor_fields_3.clone(), at_2);
                                                            continue;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let field_names_2 = {
                                                            let source_ml_source2_concat_value265 = field_names;
                                                            {
                                                                let source_ml_source2_concat_value266 = vec![field.clone()];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value265), source_ml_source2_concat_value266)
                                                            }
                                                        };
                                                        {
                                                            let fields_2 = fields.concat(&((((self::ml_string::Text::from_units(vec![44, 32])).concat(&field)).concat(&self::ml_string::Text::from_units(vec![58, 32, 40]))).concat(&value_6)).concat(&self::ml_string::Text::from_units(vec![41])));
                                                            {
                                                                let at_3 = (at + 3f64);
                                                                {
                                                                    (stubs, initializations, constructor_fields, field_names, fields, at) = (stubs_3.clone(), initializations_3.clone(), constructor_fields_3.clone(), field_names_2, fields_2.clone(), at_3);
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
        } else {
            self::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Done(stubs.clone(), initializations.clone(), constructor_fields.clone(), fields.clone())
        };
    }
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop49(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values47: Vec<Vec<f64>>, mut source_ml_source1_temporary_index47: self::ml::Big) -> Vec<self::ml_string::Text> {
    loop {
        return if (source_ml_source1_temporary_index47 < (self::ml::Big::from_u128(source_ml_source1_temporary_values47.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values47, source_ml_source1_temporary_index47.to_index());
                {
                    let names_2 = {
                        let source_ml_source2_concat_value267 = names;
                        {
                            let source_ml_source2_concat_value268 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value267), source_ml_source2_concat_value268)
                        }
                    };
                    {
                        let source_ml_source1_temporary_index47_2 = source_ml_source1_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_source1_temporary_index47) = (names_2, source_ml_source1_temporary_index47_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result {
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done,
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result {
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done,
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done,
    MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(bool),
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop52(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut method: f64, mut at: f64) -> self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    loop {
        return if (at < ((self::ml_array::optional_at(&row, self::ml_array::number_index((method + 4f64)))).clone().unwrap_or_else(|| 0f64))) {
            if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) {
                self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(true)
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
            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done
        };
    }
}
