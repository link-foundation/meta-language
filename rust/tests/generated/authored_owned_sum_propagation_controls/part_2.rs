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
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from("owned"), 0f64, String::from(""), String::from(""))) == (true)), "assertion 1");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from("actual error String"), String::from("actual error String"))) == (true)), "assertion 2");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from("actual error u8"), String::from("actual error u16"))) == (false)), "assertion 3");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from(""), String::from("actual error String"))) == (false)), "assertion 4");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), false, String::from("owned"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 5");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from("copy"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 6");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from("owned"), 1f64, String::from(""), String::from(""))) == (false)), "assertion 7");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from(""), 0f64, String::from(""), String::from(""))) == (false)), "assertion 8");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 9");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("local::Option"), String::from("local::Option"), true, String::from("owned"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 10");
    assert!(((self::accept_rust_standard_sum_question_family(String::from("std::option::Option"), String::from("std::option::Option"), String::from(""), String::from(""))) == (true)), "assertion 11");
    assert!(((self::accept_rust_standard_sum_question_family(String::from("std::result::Result"), String::from("std::result::Result"), String::from("a"), String::from("a"))) == (true)), "assertion 12");
    assert!(((self::accept_rust_standard_sum_question_family(String::from("std::result::Result"), String::from("std::result::Result"), String::from("a"), String::from("b"))) == (false)), "assertion 13");
    assert!(((self::accept_rust_standard_sum_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, false, String::from(""), String::from(""))) == (false)), "assertion 14");
    assert!(((self::accept_rust_standard_sum_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, true, String::from(""), String::from(""))) == (true)), "assertion 15");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (vec![String::from("copy"), String::from("u16"), String::from("integer exact")])), "assertion 16");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, false, true, true, String::from("u8"), String::from("u16"), 99f64)) == (vec![String::from("owned"), String::from("u16"), String::from("integer exact")])), "assertion 17");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("bool"), String::from("u16"), 90f64)) == (vec![String::from("copy"), String::from("u16"), String::from("boolean integer")])), "assertion 18");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u16"), String::from("f64"), 90f64)) == (vec![String::from("copy"), String::from("f64"), String::from("floating exact")])), "assertion 19");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u16"), String::from("u8"), 90f64)) == (Vec::<String>::new())), "assertion 20");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("usize"), String::from("u64"), 90f64)) == (Vec::<String>::new())), "assertion 21");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u64"), String::from("f64"), 90f64)) == (Vec::<String>::new())), "assertion 22");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, false, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 23");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, false, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 24");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, false, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 25");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 1f64, false, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 26");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("borrowed"), 0f64, false, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 27");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::option::Option"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 28");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("local::Result"), String::from("local::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 29");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), false, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 30");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 89f64)) == (Vec::<String>::new())), "assertion 31");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 100f64)) == (Vec::<String>::new())), "assertion 32");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u8"), 90f64)) == (Vec::<String>::new())), "assertion 33");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from("copy"), false)) == (true)), "assertion 34");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from("owned"), true)) == (true)), "assertion 35");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from("owned"), false)) == (false)), "assertion 36");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from(""), true)) == (false)), "assertion 37");
    assert!(((self::accept_rust_standard_sum_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, true, String::from("u8 actual identity"), String::from("u16 actual identity"))) == (false)), "assertion 38");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from("u8 actual identity"), String::from("u16 actual identity"))) == (false)), "assertion 39");
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
fn all_literal_owned_sum_propagation_controls() { main(); }
