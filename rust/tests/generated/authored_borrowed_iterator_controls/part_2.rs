pub fn render_primitive_method_expression(target: String, receiver: String, operation: String, object: String, arguments_text: String) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), object)), (String::from(").")))), operation)), (String::from("(")))), arguments_text)), (String::from(")")))
    } else {
        if (((target == "Rust") && (receiver == "array")) && (operation == "slice")) {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_array::slice(")), object)), (String::from(", ")))), arguments_text)), (String::from(")")))
        } else {
            if ((target != "Rust") || (receiver != "string")) {
                String::from("")
            } else {
                if (operation == "indexOf") {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".index_of(")))), arguments_text)), (String::from(")")))
                } else {
                    if (operation == "lastIndexOf") {
                        format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".last_index_of(")))), arguments_text)), (String::from(")")))
                    } else {
                        if (operation == "codePointAt") {
                            format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".code_point_at(")))), arguments_text)), (String::from(")")))
                        } else {
                            if ((operation == "replace") || (operation == "replaceAll")) {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".replace(")))), arguments_text)), (String::from(", ")))), (if (operation == "replaceAll") {
                                    String::from("true")
                                } else {
                                    String::from("false")
                                }))), (String::from(")")))
                            } else {
                                if ((operation == "padStart") || (operation == "padEnd")) {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".pad(")))), arguments_text)), (String::from(", ")))), (if (operation == "padStart") {
                                        String::from("true")
                                    } else {
                                        String::from("false")
                                    }))), (String::from(")")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".")))), operation)), (String::from("(")))), arguments_text)), (String::from(")")))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn require_primitive_method_utf16(expression: String, receiver: String) -> bool {
    ((expression == "method") && (receiver == "string"))
}

pub fn read_primitive_method_lowering_form(receiver: String, operation: String) -> String {
    if ((receiver == "array") && (operation == "at")) {
        String::from("relative-element-template")
    } else {
        if ((receiver == "string") && (operation == "at")) {
            String::from("relative-character-template")
        } else {
            if ((receiver == "string") && (operation == "charAt")) {
                String::from("character-template")
            } else {
                if ((receiver == "string") && (operation == "charCodeAt")) {
                    String::from("code-unit-template")
                } else {
                    String::from("method")
                }
            }
        }
    }
}

pub fn read_primitive_array_join_element_form(kind: String) -> String {
    if (kind == "string") {
        String::from("string")
    } else {
        if (kind == "unit") {
            String::from("empty")
        } else {
            if (kind == "option") {
                String::from("optional")
            } else {
                if (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "int")) || (kind == "nat")) {
                    String::from("text")
                } else {
                    String::from("refuse")
                }
            }
        }
    }
}

pub fn read_primitive_method_argument_obligation_feature(operation: String) -> String {
    if (operation == "join") {
        String::from("array join")
    } else {
        String::from("array identity search")
    }
}

pub fn read_primitive_array_join_operand_form(units: bool, string_element: bool, operand: String) -> String {
    if (units && ((operand == "separator") || string_element)) {
        String::from("borrow")
    } else {
        String::from("receiver")
    }
}

pub fn render_primitive_array_join_element(kind: String, absence: String, name: String, present: String, empty: String) -> String {
    {
        let form = self::read_primitive_array_join_element_form(kind.clone());
        if (form == "empty") {
            empty.clone()
        } else {
            if (form == "optional") {
                if (absence == "both") {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("match ")), name)), (String::from(" { crate::MlOptional::Value(ml_present) => ")))), present)), (String::from(", _ => ")))), empty)), (String::from(" }")))
                } else {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("match ")), name)), (String::from(" { Some(ml_present) => ")))), present)), (String::from(", None => ")))), empty)), (String::from(" }")))
                }
            } else {
                present.clone()
            }
        }
    }
}

pub fn render_primitive_array_join_expression(units: bool, string_element: bool, object: String, separator: String, element_text: String) -> String {
    if string_element {
        if units {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::join(")), object)), (String::from(", ")))), separator)), (String::from(")")))
        } else {
            format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".join(")))), separator)), (String::from(".as_str())")))
        }
    } else {
        {
            let values = format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".iter().cloned().map(|ml_value| ")))), element_text)), (String::from(").collect::<Vec<_>>()")));
            if units {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::join(&")), values)), (String::from(", ")))), separator)), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), values)), (String::from(").join(")))), separator)), (String::from(".as_str())")))
            }
        }
    }
}

pub fn accept_rust_borrowed_iterator_receiver(selected: bool, method: String, count: f64, kind: String, owner: String, path: String, arguments_count: f64, closed: bool) -> bool {
    (((((((selected && (method == "iter")) && (count == (0f64))) && (kind == "array")) && (owner == "library")) && (path == "std::vec::Vec")) && (arguments_count == (1f64))) && closed)
}

pub fn read_rust_borrowed_iterator_continuation(operation: String) -> String {
    if (operation == "map") {
        String::from("map")
    } else {
        if (operation == "chain") {
            String::from("chain")
        } else {
            if (operation == "collect") {
                String::from("collect")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_borrowed_iterator_projection_syntax(kind: String) -> String {
    if (kind == "name") {
        String::from("name")
    } else {
        if (kind == "match") {
            String::from("match")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_iterator_callback_error(selected: bool, parameters: f64, immutable: bool, reference: bool, closed: bool, captures: f64, body_projection: bool) -> String {
    if ((((((selected && (parameters == (1f64))) && immutable) && reference) && closed) && (captures == (0f64))) && body_projection) {
        String::from("")
    } else {
        String::from("a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection")
    }
}

pub fn project_rust_borrowed_iterator_callback_regions(loans: Vec<String>, existing: Vec<String>, local: Vec<String>, receiver: Vec<String>) -> Vec<String> {
    {
        let result = Vec::<String>::new();
        {
            let source_ml_source1_temporary_values1 = loans;
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s1 = self::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3(existing, local, receiver, result, source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.clone());
                    match source_ml_source2_s1.clone() {
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(result_2) => {
                            result_2
                        }
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(source_ml_source1_temporary_result3) => {
                            source_ml_source1_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_iterator_region_error(actual: f64, projected: f64, collecting: bool) -> String {
    if (!collecting && ((actual == (0f64)) || (projected == (0f64)))) {
        String::from("a readonly iterator value needs its actual live receiver-derived regions")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_iterator_once(path: String, shadowed: bool, arguments_count: f64) -> bool {
    (((path == "std::iter::once") && !shadowed) && (arguments_count == (1f64)))
}

pub fn accept_rust_borrowed_iterator_box_reference(selected: bool, canonical: bool, arguments_count: f64, reference: bool, live: bool, collecting: bool) -> bool {
    ((((selected && canonical) && (arguments_count == (0f64))) && reference) && (live || collecting))
}

pub fn read_rust_borrowed_iterator_budget_error(stages: f64, nodes: f64, regions: f64) -> String {
    {
        let source_ml_source1_temporary_values5 = vec![stages, nodes, regions];
        {
            let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6(source_ml_source1_temporary_values5, source_ml_source1_temporary_index5.clone());
                match source_ml_source2_s2.clone() {
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done => {
                        if (((stages > (64f64)) || (nodes > (100000f64))) || (regions > (65536f64))) {
                            String::from("the readonly iterator source pipeline exceeds its finite profile")
                        } else {
                            String::from("")
                        }
                    }
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6.clone()
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_iterator_collection_error(type_: bool, owner: bool, reference: bool, context: bool) -> String {
    if (((type_ && owner) && reference) && context) {
        String::from("")
    } else {
        String::from("readonly collection needs its actual matching borrowed vector result position")
    }
}

pub fn accept_rust_borrowed_iterator_production(operation: String, source: bool, result: bool, projection: bool) -> bool {
    (((self::ml_array::includes(&vec![String::from("slice"), String::from("map"), String::from("concat")], &operation.clone(), 0f64, false, |a, b| a == b) && source) && result) && projection)
}

pub fn accept_rust_borrowed_iterator_whole_slice(kinds: Vec<String>, types: Vec<String>, values: Vec<String>) -> bool {
    (((((((((((kinds.len() as f64)) == (2f64)) && (((types.len() as f64)) == (2f64))) && (((values.len() as f64)) == (2f64))) && {
        let source_ml_source2_comparison_operand3 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand4 = String::from("lit");
            match (source_ml_source2_comparison_operand3.clone(), source_ml_source2_comparison_operand4.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand5 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(1f64));
        {
            let source_ml_source2_comparison_operand6 = String::from("lit");
            match (source_ml_source2_comparison_operand5.clone(), source_ml_source2_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand7 = self::ml_array::optional_at(&types, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand8 = String::from("float");
            match (source_ml_source2_comparison_operand7.clone(), source_ml_source2_comparison_operand8.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand9 = self::ml_array::optional_at(&types, self::ml_array::number_index(1f64));
        {
            let source_ml_source2_comparison_operand10 = String::from("float");
            match (source_ml_source2_comparison_operand9.clone(), source_ml_source2_comparison_operand10.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand11 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand12 = self::read_primitive_method_default_value(String::from("array"), String::from("slice"), 0f64);
            match (source_ml_source2_comparison_operand11.clone(), source_ml_source2_comparison_operand12.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand13 = self::ml_array::optional_at(&values, self::ml_array::number_index(1f64));
        {
            let source_ml_source2_comparison_operand14 = self::read_primitive_method_default_value(String::from("array"), String::from("slice"), 1f64);
            match (source_ml_source2_comparison_operand13.clone(), source_ml_source2_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    })
}

pub fn read_rust_borrowed_iterator_snapshot_error(count: f64, text: f64) -> String {
    if (((((((!(self::ml_math::is_safe_integer(count)) || !(self::ml_math::is_safe_integer(text))) || (count < (0f64))) || (text < (0f64))) || {
        let source_ml_source2_same_value_operand15 = count;
        {
            let source_ml_source2_same_value_operand16 = (-0f64);
            (((source_ml_source2_same_value_operand15 == source_ml_source2_same_value_operand16) && ((source_ml_source2_same_value_operand15 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand15)) == ((1f64 / source_ml_source2_same_value_operand16))))) || ((source_ml_source2_same_value_operand15 != source_ml_source2_same_value_operand15) && (source_ml_source2_same_value_operand16 != source_ml_source2_same_value_operand16)))
        }
    }) || {
        let source_ml_source2_same_value_operand17 = text;
        {
            let source_ml_source2_same_value_operand18 = (-0f64);
            (((source_ml_source2_same_value_operand17 == source_ml_source2_same_value_operand18) && ((source_ml_source2_same_value_operand17 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17)) == ((1f64 / source_ml_source2_same_value_operand18))))) || ((source_ml_source2_same_value_operand17 != source_ml_source2_same_value_operand17) && (source_ml_source2_same_value_operand18 != source_ml_source2_same_value_operand18)))
        }
    }) || (count > (100000f64))) || (text > (16777216f64))) {
        String::from("readonly callback ownership snapshots exceed their finite source profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_iterator_parameter_error(name: String, raw_: String) -> String {
    if raw_.starts_with("r#") {
        String::from("")
    } else {
        if self::ml_array::includes(&vec![String::from("_"), String::from("as"), String::from("break"), String::from("const"), String::from("continue"), String::from("crate"), String::from("else"), String::from("enum"), String::from("extern"), String::from("false"), String::from("fn"), String::from("for"), String::from("if"), String::from("impl"), String::from("in"), String::from("let"), String::from("loop"), String::from("match"), String::from("mod"), String::from("move"), String::from("mut"), String::from("pub"), String::from("ref"), String::from("return"), String::from("self"), String::from("Self"), String::from("static"), String::from("struct"), String::from("super"), String::from("trait"), String::from("true"), String::from("type"), String::from("unsafe"), String::from("use"), String::from("where"), String::from("while"), String::from("async"), String::from("await"), String::from("dyn"), String::from("abstract"), String::from("become"), String::from("box"), String::from("do"), String::from("final"), String::from("gen"), String::from("macro"), String::from("override"), String::from("priv"), String::from("try"), String::from("typeof"), String::from("unsized"), String::from("virtual"), String::from("yield")], &name.clone(), 0f64, false, |a, b| a == b) {
            String::from("the readonly callback needs a fresh direct immutable binding identifier")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_iterator_binding_owner_error(value_kind: String, declared_type: bool) -> String {
    if (((value_kind == "const") || (value_kind == "ctor")) || declared_type) {
        String::from("readonly callback parameters require a fresh binder rather than a constructor or constant pattern")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_iterator_region_work_error(count: f64) -> String {
    if (((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || {
        let source_ml_source2_same_value_operand19 = count;
        {
            let source_ml_source2_same_value_operand20 = (-0f64);
            (((source_ml_source2_same_value_operand19 == source_ml_source2_same_value_operand20) && ((source_ml_source2_same_value_operand19 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand19)) == ((1f64 / source_ml_source2_same_value_operand20))))) || ((source_ml_source2_same_value_operand19 != source_ml_source2_same_value_operand19) && (source_ml_source2_same_value_operand20 != source_ml_source2_same_value_operand20)))
        }
    }) || (count > (256f64))) {
        String::from("readonly iterator region union exceeds its bounded source work profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_iterator_box_reference_error(canonical: bool, count: f64, admitted: bool, projection: bool) -> String {
    if !canonical {
        String::from("")
    } else {
        if (count != (0f64)) {
            String::from("canonical Box::as_ref expects zero arguments")
        } else {
            if (!admitted || !projection) {
                String::from("canonical Box::as_ref requires its actual live readonly receiver projection")
            } else {
                String::from("")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result {
    MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(Vec<String>),
    MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(Vec<String>),
}

pub fn source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop4(mut result: Vec<String>, mut source_ml_source1_temporary_values2: Vec<String>, mut source_ml_source1_temporary_index2: self::ml::Big) -> Vec<String> {
    loop {
        return if (source_ml_source1_temporary_index2 < (self::ml::Big::from_u128(source_ml_source1_temporary_values2.len() as u128))) {
            {
                let root = self::ml_array::at(&source_ml_source1_temporary_values2, source_ml_source1_temporary_index2.to_index());
                if !(self::ml_array::includes(&result, &root.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![root.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let source_ml_source1_temporary_index2_2 = source_ml_source1_temporary_index2.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index2) = (result_2, source_ml_source1_temporary_index2_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index2_3 = source_ml_source1_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index2 = source_ml_source1_temporary_index2_3.clone();
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

pub fn source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3(mut existing: Vec<String>, mut local: Vec<String>, mut receiver: Vec<String>, mut result: Vec<String>, mut source_ml_source1_temporary_values1: Vec<String>, mut source_ml_source1_temporary_index1: self::ml::Big) -> self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result {
    loop {
        return if (source_ml_source1_temporary_index1 < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))) {
            {
                let loan = self::ml_array::at(&source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.to_index());
                if self::ml_array::includes(&receiver, &loan.clone(), 0f64, false, |a, b| a == b) {
                    if !(self::ml_array::includes(&result, &loan.clone(), 0f64, false, |a, b| a == b)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![loan.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index1_2 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                                {
                                    (result, source_ml_source1_temporary_index1) = (result_2, source_ml_source1_temporary_index1_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index1_3 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1 = source_ml_source1_temporary_index1_3.clone();
                                continue;
                            }
                        }
                    }
                } else {
                    if (self::ml_array::includes(&local, &loan.clone(), 0f64, false, |a, b| a == b) && !(self::ml_array::includes(&existing, &loan.clone(), 0f64, false, |a, b| a == b))) {
                        {
                            let source_ml_source1_temporary_values2 = receiver.clone();
                            {
                                let source_ml_source1_temporary_index2 = self::ml::Big::from_i128(0);
                                {
                                    let result_3 = self::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop4(result, source_ml_source1_temporary_values2, source_ml_source1_temporary_index2.clone());
                                    {
                                        let source_ml_source1_temporary_index1_4 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, source_ml_source1_temporary_index1) = (result_3, source_ml_source1_temporary_index1_4.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(Vec::<String>::new())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result {
    MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done,
    MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(String),
}

pub fn source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6(mut source_ml_source1_temporary_values5: Vec<f64>, mut source_ml_source1_temporary_index5: self::ml::Big) -> self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result {
    loop {
        return if (source_ml_source1_temporary_index5 < (self::ml::Big::from_u128(source_ml_source1_temporary_values5.len() as u128))) {
            {
                let count = self::ml_array::at(&source_ml_source1_temporary_values5, source_ml_source1_temporary_index5.to_index());
                if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || {
                    let source_ml_source2_same_value_operand21 = count;
                    {
                        let source_ml_source2_same_value_operand22 = (-0f64);
                        (((source_ml_source2_same_value_operand21 == source_ml_source2_same_value_operand22) && ((source_ml_source2_same_value_operand21 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand21)) == ((1f64 / source_ml_source2_same_value_operand22))))) || ((source_ml_source2_same_value_operand21 != source_ml_source2_same_value_operand21) && (source_ml_source2_same_value_operand22 != source_ml_source2_same_value_operand22)))
                    }
                }) {
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(String::from("invalid readonly iterator source resource count"))
                } else {
                    {
                        let source_ml_source1_temporary_index5_2 = source_ml_source1_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index5 = source_ml_source1_temporary_index5_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true)) == (true)), "assertion 1");
    assert!(((self::accept_rust_borrowed_iterator_receiver(false, String::from("iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true)) == (false)), "assertion 2");
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("into_iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true)) == (false)), "assertion 3");
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("iter"), 0f64, String::from("array"), String::from("declaration"), String::from("std::vec::Vec"), 1f64, true)) == (false)), "assertion 4");
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, false)) == (false)), "assertion 5");
    assert!(((self::read_rust_borrowed_iterator_continuation(String::from("map"))) == "map"), "assertion 6");
    assert!(((self::read_rust_borrowed_iterator_continuation(String::from("copied"))) == ""), "assertion 7");
    assert!(((self::read_rust_borrowed_iterator_projection_syntax(String::from("match"))) == "match"), "assertion 8");
    assert!(((self::read_rust_borrowed_iterator_projection_syntax(String::from("let"))) == ""), "assertion 9");
    assert!(((self::read_rust_borrowed_iterator_projection_syntax(String::from("app"))) == ""), "assertion 10");
    assert!(((self::read_rust_borrowed_iterator_callback_error(true, 1f64, true, true, true, 0f64, true)) == ""), "assertion 11");
    assert!(((self::read_rust_borrowed_iterator_callback_error(true, 1f64, true, true, true, 1f64, true)) == "a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection"), "assertion 12");
    assert!(((self::read_rust_borrowed_iterator_callback_error(true, 1f64, true, false, true, 0f64, true)) == "a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection"), "assertion 13");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("parameter")], vec![String::from("receiver")], vec![String::from("parameter")], vec![String::from("receiver")])) == (vec![String::from("receiver")])), "assertion 14");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("receiver"), String::from("parameter")], vec![String::from("receiver")], vec![String::from("parameter")], vec![String::from("receiver")])) == (vec![String::from("receiver")])), "assertion 15");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("old unrelated")], vec![String::from("old unrelated")], vec![String::from("old unrelated")], vec![String::from("receiver")])) == (Vec::<String>::new())), "assertion 16");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("unknown")], Vec::<String>::new(), vec![String::from("parameter")], vec![String::from("receiver")])) == (Vec::<String>::new())), "assertion 17");
    assert!(((self::read_rust_borrowed_iterator_region_error(1f64, 1f64, false)) == ""), "assertion 18");
    assert!(((self::read_rust_borrowed_iterator_region_error(1f64, 0f64, false)) == "a readonly iterator value needs its actual live receiver-derived regions"), "assertion 19");
    assert!(((self::accept_rust_borrowed_iterator_once(String::from("std::iter::once"), false, 1f64)) == (true)), "assertion 20");
    assert!(((self::accept_rust_borrowed_iterator_once(String::from("std::iter::once"), true, 1f64)) == (false)), "assertion 21");
    assert!(((self::accept_rust_borrowed_iterator_once(String::from("my::once"), false, 1f64)) == (false)), "assertion 22");
    assert!(((self::accept_rust_borrowed_iterator_box_reference(true, true, 0f64, true, true, false)) == (true)), "assertion 23");
    assert!(((self::accept_rust_borrowed_iterator_box_reference(true, false, 0f64, true, true, false)) == (false)), "assertion 24");
    assert!(((self::accept_rust_borrowed_iterator_box_reference(true, true, 0f64, true, false, false)) == (false)), "assertion 25");
    assert!(((self::read_rust_borrowed_iterator_budget_error(64f64, 100000f64, 65536f64)) == ""), "assertion 26");
    assert!(((self::read_rust_borrowed_iterator_budget_error(65f64, 0f64, 0f64)) == "the readonly iterator source pipeline exceeds its finite profile"), "assertion 27");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, f64::NAN, 0f64)) == "invalid readonly iterator source resource count"), "assertion 28");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, 0f64, (-0f64))) == "invalid readonly iterator source resource count"), "assertion 29");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, f64::INFINITY, 0f64)) == "invalid readonly iterator source resource count"), "assertion 30");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, 0.5f64, 0f64)) == "invalid readonly iterator source resource count"), "assertion 31");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, (-1f64), 0f64)) == "invalid readonly iterator source resource count"), "assertion 32");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, 9007199254740992f64, 0f64)) == "invalid readonly iterator source resource count"), "assertion 33");
    assert!(((self::accept_rust_borrowed_iterator_production(String::from("map"), true, true, true)) == (true)), "assertion 34");
    assert!(((self::accept_rust_borrowed_iterator_production(String::from("map"), true, true, false)) == (false)), "assertion 35");
    assert!(((self::accept_rust_borrowed_iterator_whole_slice(vec![String::from("lit"), String::from("lit")], vec![String::from("float"), String::from("float")], vec![String::from("0"), String::from("Infinity")])) == (true)), "assertion 36");
    assert!(((self::accept_rust_borrowed_iterator_whole_slice(vec![String::from("lit"), String::from("lit")], vec![String::from("float"), String::from("float")], vec![String::from("1"), String::from("Infinity")])) == (false)), "assertion 37");
    assert!(((self::accept_rust_borrowed_iterator_whole_slice(vec![String::from("lit"), String::from("lit")], vec![String::from("fixed"), String::from("float")], vec![String::from("0"), String::from("Infinity")])) == (false)), "assertion 38");
    assert!(((self::read_rust_borrowed_iterator_snapshot_error(100000f64, 16777216f64)) == ""), "assertion 39");
    assert!(((self::read_rust_borrowed_iterator_snapshot_error(100001f64, 0f64)) == "readonly callback ownership snapshots exceed their finite source profile"), "assertion 40");
    assert!(((self::read_rust_borrowed_iterator_snapshot_error(0f64, (-0f64))) == "readonly callback ownership snapshots exceed their finite source profile"), "assertion 41");
    assert!(((self::read_rust_borrowed_iterator_parameter_error(String::from("field"), String::from("field"))) == ""), "assertion 42");
    assert!(((self::read_rust_borrowed_iterator_parameter_error(String::from("mut"), String::from("mut"))) == "the readonly callback needs a fresh direct immutable binding identifier"), "assertion 43");
    assert!(((self::read_rust_borrowed_iterator_parameter_error(String::from("mut"), String::from("r#mut"))) == ""), "assertion 44");
    assert!(((self::read_rust_borrowed_iterator_binding_owner_error(String::from("ctor"), false)) == "readonly callback parameters require a fresh binder rather than a constructor or constant pattern"), "assertion 45");
    assert!(((self::read_rust_borrowed_iterator_binding_owner_error(String::from(""), true)) == "readonly callback parameters require a fresh binder rather than a constructor or constant pattern"), "assertion 46");
    assert!(((self::read_rust_borrowed_iterator_binding_owner_error(String::from("fn"), false)) == ""), "assertion 47");
    assert!(((self::read_rust_borrowed_iterator_region_work_error(256f64)) == ""), "assertion 48");
    assert!(((self::read_rust_borrowed_iterator_region_work_error(257f64)) == "readonly iterator region union exceeds its bounded source work profile"), "assertion 49");
    assert!(((self::read_rust_borrowed_iterator_region_work_error((-0f64))) == "readonly iterator region union exceeds its bounded source work profile"), "assertion 50");
    assert!(((self::read_rust_borrowed_iterator_region_work_error(f64::NAN)) == "readonly iterator region union exceeds its bounded source work profile"), "assertion 51");
    assert!(((self::read_rust_borrowed_iterator_box_reference_error(true, 1f64, false, false)) == "canonical Box::as_ref expects zero arguments"), "assertion 52");
    assert!(((self::read_rust_borrowed_iterator_box_reference_error(true, 0f64, false, false)) == "canonical Box::as_ref requires its actual live readonly receiver projection"), "assertion 53");
    assert!(((self::read_rust_borrowed_iterator_box_reference_error(false, 1f64, false, false)) == ""), "assertion 54");
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
fn all_literal_borrowed_iterator_controls() { main(); }
