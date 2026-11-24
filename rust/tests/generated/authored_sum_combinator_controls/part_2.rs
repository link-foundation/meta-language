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

pub fn read_rust_standard_sum_combinator_arity(owner: String, method: String) -> f64 {
    if ((self::read_rust_standard_sum_source_argument_count(owner.clone())) < (0f64)) {
        (-1f64)
    } else {
        if ((((method == "and") || (method == "or")) || (method == "or_else")) || ((owner == "std::option::Option") && (method == "xor"))) {
            1f64
        } else {
            (-1f64)
        }
    }
}

pub fn read_rust_standard_sum_combinator_capture_roles(method: String) -> Vec<String> {
    if (method == "or_else") {
        vec![String::from("receiver"), String::from("callback")]
    } else {
        if (((method == "and") || (method == "or")) || (method == "xor")) {
            vec![String::from("receiver"), String::from("argument")]
        } else {
            Vec::<String>::new()
        }
    }
}

pub fn accept_rust_standard_sum_combinator_ownership(role: String, method: String) -> bool {
    (self::ml_array::includes(&vec![String::from("and"), String::from("or"), String::from("xor"), String::from("or_else")], &method.clone(), 0f64, false, |a, b| a == b) && ((role == "copy") || ((role == "owned") && (method != "or_else"))))
}

pub fn accept_rust_standard_sum_combinator_receiver_context(owner: String, method: String) -> bool {
    ((owner == "std::option::Option") && (((method == "or") || (method == "xor")) || (method == "or_else")))
}

pub fn read_rust_standard_sum_combinator_retained_arguments(owner: String, method: String) -> Vec<f64> {
    if ((self::read_rust_standard_sum_combinator_arity(owner.clone(), method.clone())) < (0f64)) {
        Vec::<f64>::new()
    } else {
        if (method == "and") {
            if (owner == "std::result::Result") {
                vec![1f64]
            } else {
                Vec::<f64>::new()
            }
        } else {
            vec![0f64]
        }
    }
}

pub fn read_rust_standard_sum_combinator_callback_argument(owner: String, method: String) -> f64 {
    if (((self::read_rust_standard_sum_combinator_arity(owner.clone(), method.clone())) < (0f64)) || (method != "or_else")) {
        (-2f64)
    } else {
        if (owner == "std::option::Option") {
            (-1f64)
        } else {
            1f64
        }
    }
}

pub fn plan_rust_standard_sum_combinator_right_arguments(owner: String, method: String, variant: String) -> Vec<String> {
    if (owner != "std::result::Result") {
        Vec::<String>::new()
    } else {
        if ((method == "and") && (variant == "Ok")) {
            vec![String::from("argument"), String::from("left error")]
        } else {
            if ((method == "or") && (variant == "Err")) {
                vec![String::from("left value"), String::from("argument")]
            } else {
                Vec::<String>::new()
            }
        }
    }
}

pub fn read_rust_standard_sum_combinator_branch(owner: String, left: String, method: String, right: String) -> String {
    if ((self::read_rust_standard_sum_combinator_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        {
            let field = self::read_rust_standard_sum_source_field_argument(owner.clone(), left.clone());
            if (field == ((-2f64))) {
                String::from("")
            } else {
                if (method == "and") {
                    if (field == (0f64)) {
                        String::from("right")
                    } else {
                        String::from("construct failure")
                    }
                } else {
                    if ((method == "or") || (method == "or_else")) {
                        if (field == (0f64)) {
                            if (owner == "std::option::Option") {
                                String::from("left")
                            } else {
                                String::from("construct success")
                            }
                        } else {
                            if (method == "or_else") {
                                String::from("callback")
                            } else {
                                String::from("right")
                            }
                        }
                    } else {
                        {
                            let other = self::read_rust_standard_sum_source_field_argument(owner.clone(), right.clone());
                            if (other == ((-2f64))) {
                                String::from("")
                            } else {
                                if ((field == (0f64)) && (other != (0f64))) {
                                    String::from("left")
                                } else {
                                    if ((field != (0f64)) && (other == (0f64))) {
                                        String::from("right")
                                    } else {
                                        String::from("construct failure")
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

pub fn read_rust_standard_sum_combinator_constructor(owner: String, success: bool) -> String {
    {
        let variants = self::read_rust_standard_sum_source_constructors(owner.clone());
        {
            let source_ml_source1_temporary_values6 = variants;
            {
                let source_ml_source1_temporary_index6 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s22 = self::source_ml_source1_temporary_read_rust_standard_sum_combinator_constructor_loop7(owner.clone(), success, source_ml_source1_temporary_values6, source_ml_source1_temporary_index6.clone());
                    match source_ml_source2_s22.clone() {
                        self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Done => {
                            String::from("")
                        }
                        self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Return(source_ml_source1_temporary_result7) => {
                            source_ml_source1_temporary_result7.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_combinator_version_error(owner: String, method: String, minor: f64) -> String {
    if (((self::read_rust_standard_sum_combinator_arity(owner.clone(), method.clone())) < (0f64)) || ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64)))) {
        String::from("")
    } else {
        String::from("standard sum combinator source semantics are uncovered outside Rust >=1.90.0 <1.100.0")
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
                let source_ml_source2_comparison_operand23 = self::ml_array::optional_at(&direct_copies, self::ml_array::number_index(at));
                {
                    let source_ml_source2_comparison_operand24 = true;
                    !(match (source_ml_source2_comparison_operand23.clone(), source_ml_source2_comparison_operand24) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result {
    MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Done,
    MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Return(String),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_combinator_constructor_loop7(mut owner: String, mut success: bool, mut source_ml_source1_temporary_values6: Vec<String>, mut source_ml_source1_temporary_index6: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result {
    loop {
        return if (source_ml_source1_temporary_index6 < (self::ml::Big::from_u128(source_ml_source1_temporary_values6.len() as u128))) {
            {
                let variant = self::ml_array::at(&source_ml_source1_temporary_values6, source_ml_source1_temporary_index6.to_index());
                if ((((self::read_rust_standard_sum_source_field_argument(owner.clone(), variant.clone())) == (0f64))) == success) {
                    self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Return(variant.clone())
                } else {
                    {
                        let source_ml_source1_temporary_index6_2 = source_ml_source1_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index6 = source_ml_source1_temporary_index6_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Result::MlSource1TemporaryReadRustStandardSumCombinatorConstructorLoop7Done
        };
    }
}

fn ml_main() {
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_arity(String::from("std::option::Option"), String::from("and")), 1f64), "assertion 1");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_arity(String::from("std::option::Option"), String::from("xor")), 1f64), "assertion 2");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_arity(String::from("std::result::Result"), String::from("or_else")), 1f64), "assertion 3");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_arity(String::from("std::result::Result"), String::from("xor")), (-1f64)), "assertion 4");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_arity(String::from("local::Option"), String::from("or")), (-1f64)), "assertion 5");
    assert!(((self::read_rust_standard_sum_combinator_capture_roles(String::from("or"))) == (vec![String::from("receiver"), String::from("argument")])), "assertion 6");
    assert!(((self::read_rust_standard_sum_combinator_capture_roles(String::from("or_else"))) == (vec![String::from("receiver"), String::from("callback")])), "assertion 7");
    assert!(((self::accept_rust_standard_sum_combinator_receiver_context(String::from("std::option::Option"), String::from("or"))) == (true)), "assertion 8");
    assert!(((self::accept_rust_standard_sum_combinator_receiver_context(String::from("std::option::Option"), String::from("and"))) == (false)), "assertion 9");
    assert!(((self::accept_rust_standard_sum_combinator_receiver_context(String::from("std::result::Result"), String::from("or"))) == (false)), "assertion 10");
    assert!(((self::read_rust_standard_sum_combinator_retained_arguments(String::from("std::option::Option"), String::from("and"))) == (Vec::<f64>::new())), "assertion 11");
    assert!(((self::read_rust_standard_sum_combinator_retained_arguments(String::from("std::option::Option"), String::from("xor"))) == (vec![0f64])), "assertion 12");
    assert!(((self::read_rust_standard_sum_combinator_retained_arguments(String::from("std::result::Result"), String::from("and"))) == (vec![1f64])), "assertion 13");
    assert!(((self::read_rust_standard_sum_combinator_retained_arguments(String::from("std::result::Result"), String::from("or_else"))) == (vec![0f64])), "assertion 14");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_callback_argument(String::from("std::option::Option"), String::from("or_else")), (-1f64)), "assertion 15");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_combinator_callback_argument(String::from("std::result::Result"), String::from("or_else")), 1f64), "assertion 16");
    assert!(((self::plan_rust_standard_sum_combinator_right_arguments(String::from("std::result::Result"), String::from("and"), String::from("Ok"))) == (vec![String::from("argument"), String::from("left error")])), "assertion 17");
    assert!(((self::plan_rust_standard_sum_combinator_right_arguments(String::from("std::result::Result"), String::from("or"), String::from("Err"))) == (vec![String::from("left value"), String::from("argument")])), "assertion 18");
    assert!(((self::plan_rust_standard_sum_combinator_right_arguments(String::from("std::result::Result"), String::from("or"), String::from("Ok"))) == (Vec::<String>::new())), "assertion 19");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("Some"), String::from("and"), String::from(""))) == "right"), "assertion 20");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("None"), String::from("and"), String::from(""))) == "construct failure"), "assertion 21");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("Some"), String::from("or"), String::from(""))) == "left"), "assertion 22");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("None"), String::from("or_else"), String::from(""))) == "callback"), "assertion 23");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("Some"), String::from("xor"), String::from("None"))) == "left"), "assertion 24");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("None"), String::from("xor"), String::from("Some"))) == "right"), "assertion 25");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("Some"), String::from("xor"), String::from("Some"))) == "construct failure"), "assertion 26");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::option::Option"), String::from("None"), String::from("xor"), String::from("None"))) == "construct failure"), "assertion 27");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::result::Result"), String::from("Err"), String::from("and"), String::from(""))) == "construct failure"), "assertion 28");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::result::Result"), String::from("Ok"), String::from("or"), String::from(""))) == "construct success"), "assertion 29");
    assert!(((self::read_rust_standard_sum_combinator_branch(String::from("std::result::Result"), String::from("Err"), String::from("or_else"), String::from(""))) == "callback"), "assertion 30");
    assert!(((self::read_rust_standard_sum_combinator_constructor(String::from("std::option::Option"), false)) == "None"), "assertion 31");
    assert!(((self::read_rust_standard_sum_combinator_constructor(String::from("std::result::Result"), true)) == "Ok"), "assertion 32");
    assert!(((self::read_rust_standard_sum_combinator_version_error(String::from("std::result::Result"), String::from("and"), 90f64)) == ""), "assertion 33");
    assert!(((self::read_rust_standard_sum_combinator_version_error(String::from("std::option::Option"), String::from("xor"), 99f64)) == ""), "assertion 34");
    assert!(((self::read_rust_standard_sum_combinator_version_error(String::from("std::option::Option"), String::from("or"), 100f64)) == "standard sum combinator source semantics are uncovered outside Rust >=1.90.0 <1.100.0"), "assertion 35");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("copy"), String::from("or_else"))) == (true)), "assertion 36");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("owned"), String::from("and"))) == (true)), "assertion 37");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("owned"), String::from("or"))) == (true)), "assertion 38");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("owned"), String::from("xor"))) == (true)), "assertion 39");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("owned"), String::from("or_else"))) == (false)), "assertion 40");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("reference"), String::from("and"))) == (false)), "assertion 41");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from(""), String::from("or"))) == (false)), "assertion 42");
    assert!(((self::accept_rust_standard_sum_combinator_ownership(String::from("copy"), String::from("unknown"))) == (false)), "assertion 43");
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
fn all_literal_sum_combinator_controls() { main(); }
