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

pub fn read_rust_standard_sum_source_depth(children: Vec<f64>, sum: bool) -> f64 {
    {
        let depth = 0f64;
        {
            let source_ml_source1_temporary_values1 = children;
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s19 = self::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(depth, source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.clone());
                    match source_ml_source2_s19.clone() {
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
        let source_ml_source1_temporary_values3 = vec![nodes, units, additional_nodes, additional_units];
        {
            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s20 = self::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(source_ml_source1_temporary_values3, source_ml_source1_temporary_index3.clone());
                match source_ml_source2_s20.clone() {
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

pub fn accept_rust_standard_sum_source_arguments(owner: String, identities: Vec<String>, direct_copies: Vec<bool>) -> bool {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner.clone());
        if (((count < (0f64)) || (((identities.len() as f64)) != count)) || (((direct_copies.len() as f64)) != count)) {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s21 = self::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(identities, direct_copies, count, at);
                    match source_ml_source2_s21.clone() {
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

pub fn read_rust_standard_sum_source_constructors(owner: String) -> Vec<String> {
    if (owner == "std::option::Option") {
        vec![String::from("None"), String::from("Some")]
    } else {
        if (owner == "std::result::Result") {
            vec![String::from("Ok"), String::from("Err")]
        } else {
            Vec::<String>::new()
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

pub fn read_rust_standard_sum_primitive_question_conversion_plan(operand_owner: String, return_owner: String, ordinary: bool, role: String, wrappers: f64, direct_copy: bool, source_known: bool, target_known: bool, source: String, target: String, minor: f64) -> Vec<String> {
    if ((((((((!ordinary || (operand_owner != "std::result::Result")) || (return_owner != "std::result::Result")) || (wrappers != (0f64))) || !source_known) || !target_known) || !(self::ml_math::is_integer(minor))) || (minor < (90f64))) || (minor >= (100f64))) {
        Vec::<String>::new()
    } else {
        if ((source == target) || ((role != "owned") && ((role != "copy") || !direct_copy))) {
            Vec::<String>::new()
        } else {
            {
                let plan = self::plan_rust_primitive_from(target.clone(), source.clone());
                if (plan == "") {
                    Vec::<String>::new()
                } else {
                    vec![role.clone(), target.clone(), plan.clone()]
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

pub fn read_rust_standard_sum_predicate_arity(owner: String, method: String) -> f64 {
    if ((self::read_rust_standard_sum_source_argument_count(owner.clone())) < (0f64)) {
        (-1f64)
    } else {
        if ((owner == "std::option::Option") && ((method == "is_some_and") || (method == "is_none_or"))) {
            1f64
        } else {
            if ((owner == "std::result::Result") && ((method == "is_ok_and") || (method == "is_err_and"))) {
                1f64
            } else {
                if ((method == "map_or") || (method == "map_or_else")) {
                    2f64
                } else {
                    (-1f64)
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_predicate_capture_roles(method: String) -> Vec<String> {
    if (method == "map_or") {
        vec![String::from("receiver"), String::from("default"), String::from("mapper")]
    } else {
        if (method == "map_or_else") {
            vec![String::from("receiver"), String::from("fallback"), String::from("mapper")]
        } else {
            if self::ml_array::includes(&vec![String::from("is_some_and"), String::from("is_none_or"), String::from("is_ok_and"), String::from("is_err_and")], &method.clone(), 0f64, false, |a, b| a == b) {
                vec![String::from("receiver"), String::from("predicate")]
            } else {
                Vec::<String>::new()
            }
        }
    }
}

pub fn read_rust_standard_sum_predicate_callback_argument(owner: String, method: String, role: String) -> f64 {
    if ((self::read_rust_standard_sum_predicate_arity(owner.clone(), method.clone())) < (0f64)) {
        (-2f64)
    } else {
        if (role == "mapper") {
            if ((method == "map_or") || (method == "map_or_else")) {
                0f64
            } else {
                (-2f64)
            }
        } else {
            if (role == "fallback") {
                if (method == "map_or_else") {
                    if (owner == "std::option::Option") {
                        (-1f64)
                    } else {
                        1f64
                    }
                } else {
                    (-2f64)
                }
            } else {
                if ((role == "predicate") && ((self::read_rust_standard_sum_predicate_arity(owner.clone(), method.clone())) == (1f64))) {
                    if (method == "is_err_and") {
                        1f64
                    } else {
                        0f64
                    }
                } else {
                    (-2f64)
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_predicate_branch(owner: String, variant: String, method: String) -> String {
    if ((self::read_rust_standard_sum_predicate_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        {
            let field = self::read_rust_standard_sum_source_field_argument(owner.clone(), variant.clone());
            if (field == ((-2f64))) {
                String::from("")
            } else {
                if ((method == "map_or") || (method == "map_or_else")) {
                    if (field == (0f64)) {
                        String::from("mapper")
                    } else {
                        if (method == "map_or") {
                            String::from("default")
                        } else {
                            String::from("fallback")
                        }
                    }
                } else {
                    if (field == (self::read_rust_standard_sum_predicate_callback_argument(owner.clone(), method.clone(), String::from("predicate")))) {
                        String::from("predicate")
                    } else {
                        if (method == "is_none_or") {
                            String::from("true")
                        } else {
                            String::from("false")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_predicate_version_error(owner: String, method: String, minor: f64) -> String {
    if (((self::read_rust_standard_sum_predicate_arity(owner.clone(), method.clone())) < (0f64)) || ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64)))) {
        String::from("")
    } else {
        String::from("standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0")
    }
}

pub fn read_rust_standard_sum_predicate_borrow_error(owner: String, method: String) -> String {
    if ((owner == "std::option::Option") && (method == "filter")) {
        String::from("filter needs its actual borrowed predicate parameter and complete reference lifetime model")
    } else {
        String::from("")
    }
}

pub fn read_rust_standard_sum_predicate_deferred_error(kind: String, pending: bool, parameters: f64, expected: f64, layers: f64, identity: bool) -> String {
    if (((((kind == "lambda") && pending) && (parameters == expected)) && (layers == (0f64))) && identity) {
        String::from("")
    } else {
        String::from("an unfinished callback needs its actual parameter origins, closure source identity and independently established sibling result")
    }
}

pub fn read_rust_standard_sum_predicate_result_source(first: bool, mapper: bool) -> String {
    if first {
        String::from("first result")
    } else {
        if mapper {
            String::from("mapper result")
        } else {
            String::from("")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(mut depth: f64, mut source_ml_source1_temporary_values1: Vec<f64>, mut source_ml_source1_temporary_index1: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if (source_ml_source1_temporary_index1 < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))) {
            {
                let child = self::ml_array::at(&source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.to_index());
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

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(mut source_ml_source1_temporary_values3: Vec<f64>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if (source_ml_source1_temporary_index3 < (self::ml::Big::from_u128(source_ml_source1_temporary_values3.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values3, source_ml_source1_temporary_index3.to_index());
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

pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(mut identities: Vec<String>, mut direct_copies: Vec<bool>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if (at < count) {
            if ((((self::ml_array::optional_at(&identities, self::ml_array::number_index(at))).clone().unwrap_or_else(|| String::from(""))) == "") || {
                let source_ml_source2_comparison_operand22 = self::ml_array::optional_at(&direct_copies, self::ml_array::number_index(at));
                {
                    let source_ml_source2_comparison_operand23 = true;
                    !(match (source_ml_source2_comparison_operand22.clone(), source_ml_source2_comparison_operand23) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(false)
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
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done
        };
    }
}

fn ml_main() {
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_arity(String::from("std::option::Option"), String::from("is_some_and")), 1f64), "assertion 1");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_arity(String::from("std::option::Option"), String::from("is_none_or")), 1f64), "assertion 2");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_arity(String::from("std::result::Result"), String::from("is_ok_and")), 1f64), "assertion 3");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_arity(String::from("std::result::Result"), String::from("is_err_and")), 1f64), "assertion 4");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_arity(String::from("std::option::Option"), String::from("map_or")), 2f64), "assertion 5");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_arity(String::from("std::result::Result"), String::from("map_or_else")), 2f64), "assertion 6");
    assert!(((self::read_rust_standard_sum_predicate_capture_roles(String::from("is_some_and"))) == (vec![String::from("receiver"), String::from("predicate")])), "assertion 7");
    assert!(((self::read_rust_standard_sum_predicate_capture_roles(String::from("map_or"))) == (vec![String::from("receiver"), String::from("default"), String::from("mapper")])), "assertion 8");
    assert!(((self::read_rust_standard_sum_predicate_capture_roles(String::from("map_or_else"))) == (vec![String::from("receiver"), String::from("fallback"), String::from("mapper")])), "assertion 9");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_callback_argument(String::from("std::option::Option"), String::from("map_or"), String::from("mapper")), 0f64), "assertion 10");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_callback_argument(String::from("std::option::Option"), String::from("map_or_else"), String::from("fallback")), (-1f64)), "assertion 11");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_callback_argument(String::from("std::result::Result"), String::from("map_or_else"), String::from("fallback")), 1f64), "assertion 12");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_predicate_callback_argument(String::from("std::result::Result"), String::from("is_err_and"), String::from("predicate")), 1f64), "assertion 13");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::option::Option"), String::from("Some"), String::from("is_some_and"))) == "predicate"), "assertion 14");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::option::Option"), String::from("None"), String::from("is_some_and"))) == "false"), "assertion 15");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::result::Result"), String::from("Ok"), String::from("is_err_and"))) == "false"), "assertion 16");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::result::Result"), String::from("Err"), String::from("is_err_and"))) == "predicate"), "assertion 17");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::option::Option"), String::from("Some"), String::from("map_or"))) == "mapper"), "assertion 18");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::option::Option"), String::from("None"), String::from("map_or"))) == "default"), "assertion 19");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::result::Result"), String::from("Ok"), String::from("map_or_else"))) == "mapper"), "assertion 20");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::result::Result"), String::from("Err"), String::from("map_or_else"))) == "fallback"), "assertion 21");
    assert!(((self::read_rust_standard_sum_predicate_branch(String::from("std::option::Option"), String::from("None"), String::from("is_none_or"))) == "true"), "assertion 22");
    assert!(((self::read_rust_standard_sum_predicate_version_error(String::from("std::option::Option"), String::from("map_or"), 90f64)) == ""), "assertion 23");
    assert!(((self::read_rust_standard_sum_predicate_version_error(String::from("std::result::Result"), String::from("is_err_and"), 99f64)) == ""), "assertion 24");
    assert!(((self::read_rust_standard_sum_predicate_version_error(String::from("std::option::Option"), String::from("map_or_else"), 100f64)) == "standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0"), "assertion 25");
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
fn all_literal_owned_sum_predicate_branch_controls() { main(); }
