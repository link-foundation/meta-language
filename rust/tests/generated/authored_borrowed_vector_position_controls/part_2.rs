pub fn plan_rust_primitive_from(target: String, source: String) -> String {
    if (!(self::accept_rust_primitive_from_method(target.clone(), String::from("from"))) || !(self::accept_rust_primitive_from_method(source.clone(), String::from("from")))) {
        String::from("")
    } else {
        if (target == source) {
            String::from("identity")
        } else {
            if (((source == "bool") && (target != "bool")) && (target != "f64")) {
                String::from("boolean integer")
            } else {
                if ((target == "f64") && self::ml_array::includes(&{ let ml_reference_value17 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i8"), String::from("i16"), String::from("i32"), String::from("u8"), String::from("u16"), String::from("u32")]))).borrow().clone(); ml_reference_value17 }, &source.clone(), 0f64, false, |a, b| a == b)) {
                    String::from("floating exact")
                } else {
                    {
                        let admitted = (((((((((source == "u8") && self::ml_array::includes(&{ let ml_reference_value18 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u16"), String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value18 }, &target.clone(), 0f64, false, |a, b| a == b)) || ((source == "u16") && self::ml_array::includes(&{ let ml_reference_value19 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i32"), String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value19 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u32") && self::ml_array::includes(&{ let ml_reference_value20 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u64"), String::from("u128"), String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value20 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u64") && self::ml_array::includes(&{ let ml_reference_value21 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u128"), String::from("i128")]))).borrow().clone(); ml_reference_value21 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i8") && self::ml_array::includes(&{ let ml_reference_value22 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value22 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i16") && self::ml_array::includes(&{ let ml_reference_value23 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value23 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i32") && self::ml_array::includes(&{ let ml_reference_value24 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value24 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i64") && (target == "i128")));
                        if admitted {
                            String::from("integer exact")
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_owner(path: String, standard_definition: bool, layers: f64) -> String {
    if (!standard_definition || (layers != (0f64))) {
        String::from("")
    } else {
        if ((path == "std::option::Option") || (path == "core::option::Option")) {
            String::from("std::option::Option")
        } else {
            if ((path == "std::result::Result") || (path == "core::result::Result")) {
                String::from("std::result::Result")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_standard_sum_source_path_owner(path: String) -> String {
    if self::ml_array::includes(&{ let ml_reference_value25 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Option"), String::from("std::option::Option"), String::from("core::option::Option"), String::from("None"), String::from("Some"), String::from("Option::None"), String::from("Option::Some"), String::from("std::option::Option::None"), String::from("std::option::Option::Some"), String::from("core::option::Option::None"), String::from("core::option::Option::Some")]))).borrow().clone(); ml_reference_value25 }, &path.clone(), 0f64, false, |a, b| a == b) {
        String::from("std::option::Option")
    } else {
        if self::ml_array::includes(&{ let ml_reference_value26 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Result"), String::from("std::result::Result"), String::from("core::result::Result"), String::from("Ok"), String::from("Err"), String::from("Result::Ok"), String::from("Result::Err"), String::from("std::result::Result::Ok"), String::from("std::result::Result::Err"), String::from("core::result::Result::Ok"), String::from("core::result::Result::Err")]))).borrow().clone(); ml_reference_value26 }, &path.clone(), 0f64, false, |a, b| a == b) {
            String::from("std::result::Result")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_standard_sum_source_path_constructor(path: String) -> String {
    if self::ml_array::includes(&{ let ml_reference_value27 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("None"), String::from("Option::None"), String::from("std::option::Option::None"), String::from("core::option::Option::None")]))).borrow().clone(); ml_reference_value27 }, &path.clone(), 0f64, false, |a, b| a == b) {
        String::from("None")
    } else {
        if self::ml_array::includes(&{ let ml_reference_value28 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Some"), String::from("Option::Some"), String::from("std::option::Option::Some"), String::from("core::option::Option::Some")]))).borrow().clone(); ml_reference_value28 }, &path.clone(), 0f64, false, |a, b| a == b) {
            String::from("Some")
        } else {
            if self::ml_array::includes(&{ let ml_reference_value29 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Ok"), String::from("Result::Ok"), String::from("std::result::Result::Ok"), String::from("core::result::Result::Ok")]))).borrow().clone(); ml_reference_value29 }, &path.clone(), 0f64, false, |a, b| a == b) {
                String::from("Ok")
            } else {
                if self::ml_array::includes(&{ let ml_reference_value30 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Err"), String::from("Result::Err"), String::from("std::result::Result::Err"), String::from("core::result::Result::Err")]))).borrow().clone(); ml_reference_value30 }, &path.clone(), 0f64, false, |a, b| a == b) {
                    String::from("Err")
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_instantiation_limit() -> f64 {
    256f64
}

pub fn read_rust_standard_sum_source_depth_limit() -> f64 {
    16f64
}

pub fn read_rust_standard_sum_source_specialization_name(ordinal: f64, collision: f64) -> String {
    if (((((!(self::ml_math::is_integer(ordinal)) || (ordinal < (0f64))) || (ordinal >= (256f64))) || !(self::ml_math::is_integer(collision))) || (collision < (0f64))) || (collision >= (4096f64))) {
        String::from("")
    } else {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_standard_sum_value")), ({
            let source_ml_source2_string_argument37 = ordinal;
            self::ml_number::js_number(source_ml_source2_string_argument37)
        }))), (String::from("_")))), ({
            let source_ml_source2_string_argument38 = collision;
            self::ml_number::js_number(source_ml_source2_string_argument38)
        }))
    }
}

pub fn read_rust_standard_sum_source_phase(header: bool, complete: bool, copies: bool) -> String {
    if header {
        String::from("provisional")
    } else {
        if (complete && copies) {
            String::from("admitted")
        } else {
            String::from("refuse")
        }
    }
}

pub fn read_rust_standard_sum_source_depth(children: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, sum: bool) -> f64 {
    {
        let depth = 0f64;
        {
            let source_ml_source1_temporary_values1 = children.clone();
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s39 = self::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(depth, source_ml_source1_temporary_values1.clone(), source_ml_source1_temporary_index1.clone());
                    match source_ml_source2_s39.clone() {
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth_2) => {
                            {
                                let result = (depth_2 + if sum {
                                    1f64
                                } else {
                                    0f64
                                });
                                if (result <= (16f64)) {
                                    result
                                } else {
                                    (-1f64)
                                }
                            }
                        }
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_materialization_error(nodes: f64, units: f64, additional_nodes: f64, additional_units: f64) -> String {
    {
        let source_ml_source1_temporary_values3 = std::rc::Rc::new(std::cell::RefCell::new(vec![nodes, units, additional_nodes, additional_units]));
        {
            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s40 = self::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(source_ml_source1_temporary_values3.clone(), source_ml_source1_temporary_index3.clone());
                match source_ml_source2_s40.clone() {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done => {
                        if ((((nodes > (1048576f64)) || (units > (8388608f64))) || (additional_nodes > ((1048576f64 - nodes)))) || (additional_units > ((8388608f64 - units)))) {
                            String::from("the complete standard sum source metadata exceeds its materialization budget")
                        } else {
                            String::from("")
                        }
                    }
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4.clone()
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_argument_count(owner: String) -> f64 {
    if (owner == "std::option::Option") {
        1f64
    } else {
        if (owner == "std::result::Result") {
            2f64
        } else {
            (-1f64)
        }
    }
}

pub fn accept_rust_standard_sum_source_arguments(owner: String, identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, direct_copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> bool {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner.clone());
        if (((count < (0f64)) || (({ let ml_reference_array31 = identities.clone(); let ml_reference_projection31 = ml_reference_array31.borrow().len() as f64; ml_reference_projection31 }) != count)) || (({ let ml_reference_array32 = direct_copies.clone(); let ml_reference_projection32 = ml_reference_array32.borrow().len() as f64; ml_reference_projection32 }) != count)) {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s41 = self::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(identities.clone(), direct_copies.clone(), count, at);
                    match source_ml_source2_s41.clone() {
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(source_ml_source1_temporary_result5) => {
                            source_ml_source1_temporary_result5
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_constructors(owner: String) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if (owner == "std::option::Option") {
        std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("None"), String::from("Some")]))
    } else {
        if (owner == "std::result::Result") {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Ok"), String::from("Err")]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
        }
    }
}

pub fn read_rust_standard_sum_source_field_argument(owner: String, constructor: String) -> f64 {
    if (owner == "std::option::Option") {
        if (constructor == "None") {
            (-1f64)
        } else {
            if (constructor == "Some") {
                0f64
            } else {
                (-2f64)
            }
        }
    } else {
        if (owner == "std::result::Result") {
            if (constructor == "Ok") {
                0f64
            } else {
                if (constructor == "Err") {
                    1f64
                } else {
                    (-2f64)
                }
            }
        } else {
            (-2f64)
        }
    }
}

pub fn read_rust_standard_sum_source_constructor_arity(owner: String, constructor: String) -> f64 {
    {
        let argument = self::read_rust_standard_sum_source_field_argument(owner.clone(), constructor.clone());
        if (argument == ((-2f64))) {
            (-1f64)
        } else {
            if (argument == ((-1f64))) {
                0f64
            } else {
                1f64
            }
        }
    }
}

pub fn accept_rust_standard_sum_source_inferred_constructor(owner: String, constructor: String) -> bool {
    ((owner == "std::option::Option") && (constructor == "Some"))
}

pub fn accept_rust_standard_sum_question_context(operand_owner: String, return_owner: String, ordinary_callable: bool, direct_copy_operand: bool, operand_error: String, return_error: String) -> bool {
    ((ordinary_callable && direct_copy_operand) && self::accept_rust_standard_sum_question_family(operand_owner.clone(), return_owner.clone(), operand_error.clone(), return_error.clone()))
}

pub fn accept_rust_standard_sum_question_family(operand_owner: String, return_owner: String, operand_error: String, return_error: String) -> bool {
    if (operand_owner != return_owner) {
        false
    } else {
        if (operand_owner == "std::option::Option") {
            ((operand_error == "") && (return_error == ""))
        } else {
            (((operand_owner == "std::result::Result") && (operand_error != "")) && (operand_error == return_error))
        }
    }
}

pub fn accept_rust_standard_sum_owned_question_context(operand_owner: String, return_owner: String, ordinary: bool, role: String, wrappers: f64, operand_error: String, return_error: String) -> bool {
    (((ordinary && (role == "owned")) && (wrappers == (0f64))) && self::accept_rust_standard_sum_question_family(operand_owner.clone(), return_owner.clone(), operand_error.clone(), return_error.clone()))
}

pub fn read_rust_standard_sum_primitive_question_conversion_plan(operand_owner: String, return_owner: String, ordinary: bool, role: String, wrappers: f64, direct_copy: bool, source_known: bool, target_known: bool, source: String, target: String, minor: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if ((((((((!ordinary || (operand_owner != "std::result::Result")) || (return_owner != "std::result::Result")) || (wrappers != (0f64))) || !source_known) || !target_known) || !(self::ml_math::is_integer(minor))) || (minor < (90f64))) || (minor >= (100f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
    } else {
        if ((source == target) || ((role != "owned") && ((role != "copy") || !direct_copy))) {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
        } else {
            {
                let plan = self::plan_rust_primitive_from(target.clone(), source.clone());
                if (plan == "") {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![role.clone(), target.clone(), plan.clone()]))
                }
            }
        }
    }
}

pub fn accept_rust_standard_sum_primitive_question_completion(profile: String, consumed: bool) -> bool {
    ((profile == "copy") || ((profile == "owned") && consumed))
}

pub fn read_rust_standard_sum_question_branch(owner: String, constructor: String) -> String {
    if ((self::read_rust_standard_sum_source_constructor_arity(owner.clone(), constructor.clone())) < (0f64)) {
        String::from("")
    } else {
        if (owner == "std::option::Option") {
            if (constructor == "Some") {
                String::from("continue field")
            } else {
                String::from("return absent")
            }
        } else {
            if (constructor == "Ok") {
                String::from("continue field")
            } else {
                String::from("return error field")
            }
        }
    }
}

pub fn accept_rust_source_array_owner(owner: String, shadowed: bool) -> bool {
    (!shadowed && ((owner == "Vec") || (owner == "std::vec::Vec")))
}

pub fn accept_rust_source_array_element(kind: String, indirected: bool) -> bool {
    (!indirected && (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")) || (kind == "string")))
}

pub fn accept_rust_source_array_coercion(actual: String, expected: String, borrowed: bool) -> bool {
    ((actual == expected) || ((borrowed && (actual == "owned vector")) && (expected == "immutable slice")))
}

pub fn accept_rust_source_array_result_model(model: String) -> bool {
    ((model == "") || (model == "immutable fresh scalar vector results 1"))
}

pub fn accept_rust_source_array_result(model: String, kind: String, owned: bool, fresh: bool) -> bool {
    ((((model == "immutable fresh scalar vector results 1") && owned) && fresh) && ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")))
}

pub fn accept_rust_fresh_array_result_tail(kind: String, constructor: bool, children: bool) -> bool {
    (((kind == "array") && constructor) || (((kind == "let") || (kind == "if")) && children))
}

pub fn accept_rust_borrowed_vector_completion_profiles(signature: String, profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    if ((!(self::accept_rust_borrowed_vector_signature_model(signature.clone())) || (({ let ml_reference_array33 = profiles.clone(); let ml_reference_projection33 = ml_reference_array33.borrow().len() as f64; ml_reference_projection33 }) != (11f64))) || {
        let source_ml_source2_comparison_operand44 = {
            let source_ml_source2_array_read_value42 = profiles.clone();
            {
                let source_ml_source2_array_read_index43 = 1f64;
                { let ml_reference_array34 = source_ml_source2_array_read_value42.clone(); let ml_reference_position34 = source_ml_source2_array_read_index43; let ml_reference_projection34 = self::ml_array::optional_at(&*ml_reference_array34.borrow(), self::ml_array::number_index(ml_reference_position34)); ml_reference_projection34 }
            }
        };
        {
            let source_ml_source2_comparison_operand45 = signature.clone();
            !(match (source_ml_source2_comparison_operand44.clone(), source_ml_source2_comparison_operand45.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s46 = self::source_ml_source1_temporary_accept_rust_borrowed_vector_completion_profiles_loop6(profiles.clone(), index);
                match source_ml_source2_s46.clone() {
                    self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

pub fn accept_rust_borrowed_vector_inherent_profiles(profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    if ((({ let ml_reference_array35 = profiles.clone(); let ml_reference_projection35 = ml_reference_array35.borrow().len() as f64; ml_reference_projection35 }) != (11f64)) || !(self::accept_rust_borrowed_vector_signature_model(({
        let source_ml_source2_array_read_value47 = profiles.clone();
        {
            let source_ml_source2_array_read_index48 = 2f64;
            { let ml_reference_array36 = source_ml_source2_array_read_value47.clone(); let ml_reference_position36 = source_ml_source2_array_read_index48; let ml_reference_projection36 = self::ml_array::optional_at(&*ml_reference_array36.borrow(), self::ml_array::number_index(ml_reference_position36)); ml_reference_projection36 }
        }
    }).clone().unwrap_or_else(|| String::from(""))))) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s49 = self::source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7(profiles.clone(), index);
                match source_ml_source2_s49.clone() {
                    self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(source_ml_source1_temporary_result7) => {
                        source_ml_source1_temporary_result7
                    }
                }
            }
        }
    }
}

pub fn accept_rust_borrowed_vector_source(model: String, ordinary: bool, minor: f64) -> bool {
    ((((self::accept_rust_borrowed_vector_signature_model(model.clone()) && ordinary) && self::ml_math::is_integer(minor)) && (minor >= (90f64))) && (minor < (100f64)))
}

pub fn accept_rust_borrowed_vector_signature_model(model: String) -> bool {
    (model == "Rust source function signature 3")
}

pub fn retain_rust_borrowed_vector_element_position(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, child: bool) -> bool {
    (self::ml_array::includes(&{ let ml_reference_value37 = wrappers.borrow().clone(); ml_reference_value37 }, &String::from("reference"), 0f64, false, |a, b| a == b) || child)
}

pub fn accept_rust_borrowed_vector_owned_interpretation(child: bool) -> bool {
    !child
}

pub fn accept_rust_borrowed_vector_position(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: f64, nested: bool) -> bool {
    ((((({ let ml_reference_array38 = wrappers.clone(); let ml_reference_projection38 = ml_reference_array38.borrow().len() as f64; ml_reference_projection38 }) == (1f64)) && {
        let source_ml_source2_comparison_operand52 = {
            let source_ml_source2_array_read_value50 = wrappers.clone();
            {
                let source_ml_source2_array_read_index51 = 0f64;
                { let ml_reference_array39 = source_ml_source2_array_read_value50.clone(); let ml_reference_position39 = source_ml_source2_array_read_index51; let ml_reference_projection39 = self::ml_array::optional_at(&*ml_reference_array39.borrow(), self::ml_array::number_index(ml_reference_position39)); ml_reference_projection39 }
            }
        };
        {
            let source_ml_source2_comparison_operand53 = String::from("reference");
            match (source_ml_source2_comparison_operand52.clone(), source_ml_source2_comparison_operand53.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && (lifetimes == (1f64))) && !nested)
}

pub fn read_rust_borrowed_vector_position_error(model: String, child: bool) -> String {
    if (child && !(self::accept_rust_borrowed_vector_signature_model(model.clone()))) {
        String::from("a retained array element position requires the borrowed vector source signature profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_element_position_error(kind: String, owner: String, path: String, arguments_count: f64, type_equal: bool, owner_equal: bool) -> String {
    if ((((((kind != "array") || (owner != "library")) || (path != "std::vec::Vec")) || (arguments_count != (1f64))) || !type_equal) || !owner_equal) {
        String::from("a retained array element position must match its complete canonical vector element type and owner")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_element(selected: bool, minor: f64, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>, known: bool, mutable: bool) -> bool {
    ((((((((selected && self::ml_math::is_integer(minor)) && (minor >= (90f64))) && (minor <= (99f64))) && known) && !mutable) && (({ let ml_reference_array40 = wrappers.clone(); let ml_reference_projection40 = ml_reference_array40.borrow().len() as f64; ml_reference_projection40 }) == (1f64))) && {
        let source_ml_source2_comparison_operand56 = {
            let source_ml_source2_array_read_value54 = wrappers.clone();
            {
                let source_ml_source2_array_read_index55 = 0f64;
                { let ml_reference_array41 = source_ml_source2_array_read_value54.clone(); let ml_reference_position41 = source_ml_source2_array_read_index55; let ml_reference_projection41 = self::ml_array::optional_at(&*ml_reference_array41.borrow(), self::ml_array::number_index(ml_reference_position41)); ml_reference_projection41 }
            }
        };
        {
            let source_ml_source2_comparison_operand57 = String::from("reference");
            match (source_ml_source2_comparison_operand56.clone(), source_ml_source2_comparison_operand57.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && (({ let ml_reference_array42 = lifetimes.clone(); let ml_reference_projection42 = ml_reference_array42.borrow().len() as f64; ml_reference_projection42 }) == (1f64)))
}

pub fn read_rust_borrowed_vector_position_budget_error(visits: f64, depth: f64) -> String {
    if (((((!(self::ml_math::is_safe_integer(visits)) || (visits < (0f64))) || (visits > (100000f64))) || !(self::ml_math::is_safe_integer(depth))) || (depth < (0f64))) || (depth > (128f64))) {
        String::from("retained borrowed vector element positions exceed their finite source profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_function_owner(model: String, owner: String, type_: String, paths: f64, children: f64, parameters: f64, reference: bool) -> bool {
    (self::accept_rust_borrowed_vector_signature_model(model.clone()) && self::accept_rust_retained_function_owner(String::from("Rust source function signature 2"), owner.clone(), type_.clone(), paths, children, parameters, reference))
}

pub fn read_rust_borrowed_vector_element_phase(selected: bool, reference: bool, header: bool, complete: bool, closed: bool) -> String {
    if (!selected || !reference) {
        String::from("")
    } else {
        if (header && !complete) {
            String::from("defer")
        } else {
            if (complete && closed) {
                String::from("admit")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_borrowed_vector_shape(selected: bool, kind: String, owner: String, path: String, arguments_count: f64, wrappers: f64, child: bool) -> bool {
    ((((((selected && (kind == "array")) && (owner == "library")) && (path == "std::vec::Vec")) && (arguments_count == (1f64))) && (wrappers == (0f64))) && child)
}

pub fn plan_rust_borrowed_vector_result_parameters(lifetime: String, references: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if ((({ let ml_reference_array43 = references.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 }) != ({ let ml_reference_array44 = lifetimes.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 })) || (({ let ml_reference_array45 = references.clone(); let ml_reference_projection45 = ml_reference_array45.borrow().len() as f64; ml_reference_projection45 }) > (65536f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8(lifetime.clone(), references.clone(), lifetimes.clone(), result.clone(), index);
                    if (lifetime == "static") {
                        result_2.clone()
                    } else {
                        if ((lifetime == "") || (lifetime == "_")) {
                            if (({ let ml_reference_array46 = result_2.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 }) == (1f64)) {
                                result_2.clone()
                            } else {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
                            }
                        } else {
                            if { let ml_value = { let ml_reference_array48 = result_2.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                result_2.clone()
                            } else {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_vector_production_error(same: bool, reference: bool, available: bool, collecting: bool) -> String {
    if (!same || !reference) {
        String::from("a borrowed vector element needs its actual matching immutable reference position")
    } else {
        if (!collecting && !available) {
            String::from("a borrowed vector element needs its actual live source region")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_vector_result_error(same: bool, produced: bool, allowed: bool, collecting: bool) -> String {
    if (!same || (!collecting && (!produced || !allowed))) {
        String::from("a borrowed vector result needs an actual producer and parameter-derived or static element regions")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_element_count_error(count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (65536f64))) {
        String::from("borrowed vector construction exceeds its finite actual element profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_literal_context(selected: bool, expected: bool, reference: bool, count: f64) -> bool {
    (((((selected && expected) && self::ml_math::is_safe_integer(count)) && (count >= (0f64))) && (count <= (65536f64))) && ((count == (0f64)) || reference))
}

pub fn read_rust_borrowed_vector_empty_constructor(selected: bool, owner: String, method: String, count: f64) -> String {
    if ((selected && self::accept_rust_source_array_owner(owner.clone(), false)) && (method == "new")) {
        if (count == (0f64)) {
            String::from("construct")
        } else {
            String::from("arity")
        }
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_argument_error(actual: bool, expected: bool, same: bool, owner: bool, lifetime: String, required: String) -> String {
    if ((actual != expected) || (actual && ((!same || !owner) || ((required == "static") && (lifetime != "static"))))) {
        String::from("borrowed vector arguments must retain their actual reference element owner and region contract")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_observation_error(borrowed: bool, operation: String) -> String {
    if (borrowed && !(self::ml_array::includes(&{ let ml_reference_value49 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("len"), String::from("is_empty"), String::from("transfer")]))).borrow().clone(); ml_reference_value49 }, &operation.clone(), 0f64, false, |a, b| a == b))) {
        String::from("borrowed vector observations need their independent reference-preserving source operation profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_produced_value(selected: bool, shape: bool, proven: bool, producer: bool) -> bool {
    (((selected && shape) && proven) && producer)
}

pub fn read_rust_borrowed_vector_call_region_error(reference: bool, regions: f64, collecting: bool) -> String {
    if ((!(self::ml_math::is_safe_integer(regions)) || (regions < (0f64))) || (regions > (65536f64))) {
        String::from("borrowed vector calls exceed their finite actual argument region profile")
    } else {
        if ((!collecting && reference) && (regions == (0f64))) {
            String::from("a borrowed vector call needs actual argument region provenance; anonymous temporary references need their separate promotion or extension proof")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_expression_position_model_error(model: String, language: String) -> String {
    if ((model != "Rust borrowed expression source position 1") || (language != "Rust")) {
        String::from("a borrowed expression source position needs its exact Rust source interpretation")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_expression_lifetime_scope_error(lifetime: String, bindings: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if ((((lifetime != "") && (lifetime != "_")) && (lifetime != "static")) && !(self::ml_array::includes(&{ let ml_reference_value50 = bindings.borrow().clone(); ml_reference_value50 }, &lifetime.clone(), 0f64, false, |a, b| a == b))) {
        String::from("a borrowed expression source lifetime needs its actual enclosing function binder")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_call_lifetime(result: String, arguments_: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if (result == "static") {
        String::from("static")
    } else {
        if (({ let ml_reference_array51 = arguments_.clone(); let ml_reference_projection51 = ml_reference_array51.borrow().len() as f64; ml_reference_projection51 }) == (0f64)) {
            String::from("")
        } else {
            {
                let first = ({
                    let source_ml_source2_array_read_value58 = arguments_.clone();
                    {
                        let source_ml_source2_array_read_index59 = 0f64;
                        { let ml_reference_array52 = source_ml_source2_array_read_value58.clone(); let ml_reference_position52 = source_ml_source2_array_read_index59; let ml_reference_projection52 = self::ml_array::optional_at(&*ml_reference_array52.borrow(), self::ml_array::number_index(ml_reference_position52)); ml_reference_projection52 }
                    }
                }).clone().unwrap_or_else(|| String::from(""));
                if ((first != "_") && { let ml_values53 = { let ml_reference_value54 = arguments_.borrow().clone(); ml_reference_value54 }; let ml_callback53 = { let first = first.clone(); std::rc::Rc::new(move |value: String| -> bool { (value == first) }) }; ml_values53.iter().cloned().enumerate().all(|(ml_index53, ml_value53)| ml_callback53(ml_value53)) }) {
                    first.clone()
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn read_rust_borrowed_vector_static_reference_error(selected: bool, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if ((selected && {
        let source_ml_source2_comparison_operand62 = {
            let source_ml_source2_array_read_value60 = wrappers.clone();
            {
                let source_ml_source2_array_read_index61 = 0f64;
                { let ml_reference_array55 = source_ml_source2_array_read_value60.clone(); let ml_reference_position55 = source_ml_source2_array_read_index61; let ml_reference_projection55 = self::ml_array::optional_at(&*ml_reference_array55.borrow(), self::ml_array::number_index(ml_reference_position55)); ml_reference_projection55 }
            }
        };
        {
            let source_ml_source2_comparison_operand63 = String::from("reference");
            match (source_ml_source2_comparison_operand62.clone(), source_ml_source2_comparison_operand63.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand66 = {
            let source_ml_source2_array_read_value64 = lifetimes.clone();
            {
                let source_ml_source2_array_read_index65 = 0f64;
                { let ml_reference_array56 = source_ml_source2_array_read_value64.clone(); let ml_reference_position56 = source_ml_source2_array_read_index65; let ml_reference_projection56 = self::ml_array::optional_at(&*ml_reference_array56.borrow(), self::ml_array::number_index(ml_reference_position56)); ml_reference_projection56 }
            }
        };
        {
            let source_ml_source2_comparison_operand67 = String::from("static");
            match (source_ml_source2_comparison_operand66.clone(), source_ml_source2_comparison_operand67.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) {
        String::from("static source reference demands need their independent global or parameter region proof")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_mutation_error(selected: bool) -> String {
    if selected {
        String::from("the borrowed vector source profile requires immutable bindings and explicit place-write provenance before source mutation")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_expression_position_reservation(nodes: f64, units: f64, added_nodes: f64, added_units: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let source_ml_source1_temporary_values9 = std::rc::Rc::new(std::cell::RefCell::new(vec![nodes, units, added_nodes, added_units]));
        {
            let source_ml_source1_temporary_index9 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s68 = self::source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10(source_ml_source1_temporary_values9.clone(), source_ml_source1_temporary_index9.clone());
                match source_ml_source2_s68.clone() {
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done => {
                        {
                            let next_nodes = (nodes + ((added_nodes + 2f64) * 4f64));
                            {
                                let next_units = (units + (((added_units + 13f64) + ((String::from("Rust borrowed expression source position 1")).encode_utf16().count() as f64)) * 4f64));
                                if !(self::read_rust_standard_sum_source_materialization_error(0f64, 0f64, next_nodes, next_units)).is_empty() {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![next_nodes, next_units]))
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(source_ml_source1_temporary_result10) => {
                        source_ml_source1_temporary_result10.clone()
                    }
                }
            }
        }
    }
}

pub fn plan_rust_borrowed_inherent_position(actual: std::rc::Rc<std::cell::RefCell<Vec<String>>>, target: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let actual_boxes = 0f64;
        {
            let target_boxes = 0f64;
            {
                let actual_references = 0f64;
                {
                    let target_references = 0f64;
                    {
                        let source_ml_source1_temporary_values11 = actual.clone();
                        {
                            let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s69 = self::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13(actual_boxes, actual_references, source_ml_source1_temporary_values11.clone(), source_ml_source1_temporary_index11.clone());
                                match source_ml_source2_s69.clone() {
                                    self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(actual_boxes_2, actual_references_2) => {
                                        {
                                            let source_ml_source1_temporary_values12 = target.clone();
                                            {
                                                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s70 = self::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14(target_boxes, target_references, source_ml_source1_temporary_values12.clone(), source_ml_source1_temporary_index12.clone());
                                                    match source_ml_source2_s70.clone() {
                                                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(target_boxes_2, target_references_2) => {
                                                            if (((({ let ml_reference_array57 = actual.clone(); let ml_reference_projection57 = ml_reference_array57.borrow().len() as f64; ml_reference_projection57 }) > (256f64)) || (({ let ml_reference_array58 = target.clone(); let ml_reference_projection58 = ml_reference_array58.borrow().len() as f64; ml_reference_projection58 }) > (256f64))) || (target_boxes_2 > actual_boxes_2)) {
                                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                            } else {
                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![(actual_boxes_2 - target_boxes_2), self::ml_math::max((actual_references_2 - target_references_2), 0f64), self::ml_math::max((target_references_2 - actual_references_2), 0f64)]))
                                                            }
                                                        }
                                                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(source_ml_source1_temporary_result14) => {
                                                            source_ml_source1_temporary_result14.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(source_ml_source1_temporary_result14_2) => {
                                        source_ml_source1_temporary_result14_2.clone()
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(mut depth: f64, mut source_ml_source1_temporary_values1: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index1: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if (source_ml_source1_temporary_index1 < ({ let ml_reference_array59 = source_ml_source1_temporary_values1.clone(); let ml_reference_projection59 = self::ml::Big::from_u128(ml_reference_array59.borrow().len() as u128); ml_reference_projection59 })) {
            {
                let child = {
                    let source_ml_source2_array_read_value71 = source_ml_source1_temporary_values1.clone();
                    {
                        let source_ml_source2_array_read_index72 = source_ml_source1_temporary_index1.clone();
                        { let ml_reference_array60 = source_ml_source2_array_read_value71.clone(); let ml_reference_position60 = source_ml_source2_array_read_index72.clone(); let ml_reference_projection60 = self::ml_array::at(&*ml_reference_array60.borrow(), ml_reference_position60.to_index()); ml_reference_projection60 }
                    }
                };
                if ((!(self::ml_math::is_integer(child)) || (child < (0f64))) || (child > (16f64))) {
                    self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return((-1f64))
                } else {
                    if (child > depth) {
                        {
                            let depth_2 = child;
                            {
                                let source_ml_source1_temporary_index1_2 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                                {
                                    (depth, source_ml_source1_temporary_index1) = (depth_2, source_ml_source1_temporary_index1_2.clone());
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
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done,
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(mut source_ml_source1_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if (source_ml_source1_temporary_index3 < ({ let ml_reference_array61 = source_ml_source1_temporary_values3.clone(); let ml_reference_projection61 = self::ml::Big::from_u128(ml_reference_array61.borrow().len() as u128); ml_reference_projection61 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value73 = source_ml_source1_temporary_values3.clone();
                    {
                        let source_ml_source2_array_read_index74 = source_ml_source1_temporary_index3.clone();
                        { let ml_reference_array62 = source_ml_source2_array_read_value73.clone(); let ml_reference_position62 = source_ml_source2_array_read_index74.clone(); let ml_reference_projection62 = self::ml_array::at(&*ml_reference_array62.borrow(), ml_reference_position62.to_index()); ml_reference_projection62 }
                    }
                };
                if (!(self::ml_math::is_integer(value)) || (value < (0f64))) {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String::from("invalid standard sum source materialization count"))
                } else {
                    {
                        let source_ml_source1_temporary_index3_2 = source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index3 = source_ml_source1_temporary_index3_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done,
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(bool),
}
