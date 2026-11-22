pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(mut starts: Vec<f64>, mut counts: Vec<f64>, mut targets: Vec<f64>, mut pending: Vec<f64>, mut children: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), children))) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value6 = pending;
                    {
                        let source_ml_source2_concat_value7 = vec![self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), children, at)];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value6), source_ml_source2_concat_value7)
                    }
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        (pending, at) = (pending_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(Vec<f64>, bool),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut pending: Vec<f64>, mut fields: f64, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (self::read_required_number_fact(counts.clone(), fields))) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value8 = pending;
                    {
                        let source_ml_source2_concat_value9 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), fields, index), String::from("type"))];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value8), source_ml_source2_concat_value9)
                    }
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut pending: Vec<f64>, mut constructors: f64, mut ctor: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    loop {
        return if (ctor < (self::read_required_number_fact(counts.clone(), constructors))) {
            {
                let fields = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), constructors, ctor), String::from("fields"));
                if (fields < (0f64)) {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(true)
                } else {
                    {
                        let index = 0f64;
                        {
                            let pending_2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending, fields, index);
                            {
                                let ctor_2 = (ctor + 1f64);
                                {
                                    (pending, ctor) = (pending_2, ctor_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut pending: Vec<f64>, mut name: f64, mut found: bool, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), declarations))) {
            {
                let row = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), declarations, at);
                {
                    let full_name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, String::from("fullName"));
                    if ((full_name < (0f64)) || ((self::read_required_text_fact(texts.clone(), full_name)) != (self::read_required_text_fact(texts.clone(), name)))) {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let constructors = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, String::from("ctors"));
                            if (constructors < (0f64)) {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(true)
                            } else {
                                {
                                    let found_2 = true;
                                    {
                                        let ctor = 0f64;
                                        {
                                            let source_ml_source2_s10 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending, constructors, ctor);
                                            match source_ml_source2_s10.clone() {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending_2) => {
                                                    {
                                                        let at_3 = (at + 1f64);
                                                        {
                                                            (pending, found, at) = (pending_2, found_2, at_3);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(source_ml_source1_temporary_result6) => {
                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result6)
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
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending, found)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut pending: Vec<f64>, mut seen: Vec<f64>) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    loop {
        return if { let ml_value = (pending.len() as f64); ml_value != 0f64 && !ml_value.is_nan() } {
            if ((((seen.len() as f64)) >= (100000f64)) || (((pending.len() as f64)) > (100000f64))) {
                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
            } else {
                {
                    let node = self::read_required_number_fact(pending.clone(), ((pending.len() as f64) - 1f64));
                    {
                        let pending_2 = self::ml_array::slice(&pending, 0f64, (-1f64));
                        if ((node < (0f64)) || (node >= ((starts.len() as f64)))) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                        } else {
                            if self::ml_array::includes(&seen, &node, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                                {
                                    pending = pending_2;
                                    continue;
                                }
                            } else {
                                {
                                    let seen_2 = {
                                        let source_ml_source2_concat_value11 = seen;
                                        {
                                            let source_ml_source2_concat_value12 = vec![node];
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value11), source_ml_source2_concat_value12)
                                        }
                                    };
                                    {
                                        let owner = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("kind"));
                                        if (owner < (0f64)) {
                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                        } else {
                                            {
                                                let kind = self::read_required_text_fact(texts.clone(), owner);
                                                if (kind == "record") {
                                                    {
                                                        let fields = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("fields"));
                                                        if (fields < (0f64)) {
                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                        } else {
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let source_ml_source2_s13 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), brands.clone(), pending_2, fields, at);
                                                                    match source_ml_source2_s13.clone() {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending_3) => {
                                                                            {
                                                                                (pending, seen) = (pending_3, seen_2);
                                                                                continue;
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(source_ml_source1_temporary_result3) => {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if self::ml_array::includes(&vec![String::from("array"), String::from("option"), String::from("set"), String::from("cell")], &kind.clone(), 0f64, false, |a, b| a == b) {
                                                        {
                                                            let pending_4 = {
                                                                let source_ml_source2_concat_value14 = pending_2;
                                                                {
                                                                    let source_ml_source2_concat_value15 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("element"))];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value14), source_ml_source2_concat_value15)
                                                                }
                                                            };
                                                            {
                                                                (pending, seen) = (pending_4, seen_2);
                                                                continue;
                                                            }
                                                        }
                                                    } else {
                                                        if (kind == "map") {
                                                            {
                                                                let pending_5 = {
                                                                    let source_ml_source2_concat_value16 = pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value17 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("key")), self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("value"))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value16), source_ml_source2_concat_value17)
                                                                    }
                                                                };
                                                                {
                                                                    (pending, seen) = (pending_5, seen_2);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            if (((kind == "tuple") || (kind == "union")) || (kind == "function")) {
                                                                {
                                                                    let children = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, if (kind == "tuple") {
                                                                        String::from("elements")
                                                                    } else {
                                                                        if (kind == "union") {
                                                                            String::from("alternatives")
                                                                        } else {
                                                                            String::from("params")
                                                                        }
                                                                    });
                                                                    if (children < (0f64)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let pending_6 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(starts.clone(), counts.clone(), targets.clone(), pending_2, children, at_2);
                                                                                if (kind == "function") {
                                                                                    {
                                                                                        let pending_7 = {
                                                                                            let source_ml_source2_concat_value18 = pending_6;
                                                                                            {
                                                                                                let source_ml_source2_concat_value19 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("ret"))];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value18), source_ml_source2_concat_value19)
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            (pending, seen) = (pending_7, seen_2);
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        (pending, seen) = (pending_6, seen_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (kind == "data") {
                                                                    {
                                                                        let name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("name"));
                                                                        if ((name < (0f64)) || (declarations < (0f64))) {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let found = false;
                                                                                {
                                                                                    let at_3 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s20 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s20.clone() {
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending_8, found_2) => {
                                                                                                if !found_2 {
                                                                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                                                } else {
                                                                                                    {
                                                                                                        (pending, seen) = (pending_8, seen_2);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result5) => {
                                                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if !(self::ml_array::includes(&vec![String::from("unit"), String::from("bool"), String::from("float"), String::from("int"), String::from("nat"), String::from("fixed"), String::from("string")], &kind.clone(), 0f64, false, |a, b| a == b)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            (pending, seen) = (pending_2, seen_2);
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
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(bool),
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(bool),
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut params: f64, mut at: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), params))) {
            {
                let parameter = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), params, at);
                {
                    let type_ = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), parameter, String::from("type"));
                    if self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, declarations, brands.clone()) {
                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(true)
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
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut exported: String, mut found: bool, mut index: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    loop {
        return if (index < (self::read_required_number_fact(counts.clone(), declarations))) {
            {
                let declaration = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), declarations, index);
                {
                    let name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("name"));
                    if ((name < (0f64)) || ((self::read_required_text_fact(texts.clone(), name)) != exported)) {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let kind = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("k"));
                            if ((kind < (0f64)) || ((self::read_required_text_fact(texts.clone(), kind)) != "fn")) {
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let params = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("params"));
                                    {
                                        let ret = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("ret"));
                                        if ((found || (params < (0f64))) || self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, declarations, brands.clone())) {
                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(true)
                                        } else {
                                            {
                                                let found_2 = true;
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s21 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), params, at);
                                                        match source_ml_source2_s21.clone() {
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done => {
                                                                {
                                                                    let index_4 = (index + 1f64);
                                                                    {
                                                                        (found, index) = (found_2, index_4);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(source_ml_source1_temporary_result11) => {
                                                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result11)
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
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found)
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut source_ml_source1_temporary_values8: Vec<String>, mut source_ml_source1_temporary_index8: self::ml::Big) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    loop {
        return if (source_ml_source1_temporary_index8 < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))) {
            {
                let exported = self::ml_array::at(&source_ml_source1_temporary_values8, source_ml_source1_temporary_index8.to_index());
                {
                    let found = false;
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s22 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), exported.clone(), found, index);
                            match source_ml_source2_s22.clone() {
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found_2) => {
                                    if !found_2 {
                                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(true)
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index8 = source_ml_source1_temporary_index8_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result10) => {
                                    self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_java_script_closed_class_observation_operands(String::from("toString"), String::from(""))) == "arg"), "assertion 1");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from("strictEqual"), String::from(""))) == "left right"), "assertion 2");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from(""), String::from("eq"))) == "left right"), "assertion 3");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from(""), String::from("forall"))) == "binders"), "assertion 4");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from("recordField"), String::from(""))) == ""), "assertion 5");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_class_observation_controls() { main(); }
