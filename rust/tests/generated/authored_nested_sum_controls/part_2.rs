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

pub fn read_rust_standard_sum_nested_arity(owner: String, method: String) -> f64 {
    if (((owner == "std::option::Option") && ((method == "flatten") || (method == "transpose"))) || ((owner == "std::result::Result") && (method == "transpose"))) {
        0f64
    } else {
        (-1f64)
    }
}

pub fn read_rust_standard_sum_nested_profile(owner: String, method: String, inner: String) -> String {
    if (((owner == "std::option::Option") && (method == "flatten")) && (inner == "std::option::Option")) {
        String::from("option flatten")
    } else {
        if (((owner == "std::option::Option") && (method == "transpose")) && (inner == "std::result::Result")) {
            String::from("option result transpose")
        } else {
            if (((owner == "std::result::Result") && (method == "transpose")) && (inner == "std::option::Option")) {
                String::from("result option transpose")
            } else {
                String::from("")
            }
        }
    }
}

pub fn plan_rust_standard_sum_nested_result(profile: String) -> Vec<String> {
    if (profile == "option flatten") {
        vec![String::from("inner")]
    } else {
        if (profile == "option result transpose") {
            vec![String::from("inner success"), String::from("std::option::Option"), String::from("inner error"), String::from("std::result::Result")]
        } else {
            if (profile == "result option transpose") {
                vec![String::from("inner success"), String::from("outer error"), String::from("std::result::Result"), String::from("std::option::Option")]
            } else {
                Vec::<String>::new()
            }
        }
    }
}

pub fn plan_rust_standard_sum_nested_branch(profile: String, owner: String, constructor: String, inner: bool) -> Vec<String> {
    {
        let field = self::read_rust_standard_sum_source_field_argument(owner.clone(), constructor.clone());
        if (field == ((-2f64))) {
            Vec::<String>::new()
        } else {
            if ((profile == "option flatten") && (owner == "std::option::Option")) {
                if (field == ((-1f64))) {
                    vec![String::from("result None")]
                } else {
                    if inner {
                        vec![String::from("payload"), String::from("result Some")]
                    } else {
                        vec![String::from("match inner")]
                    }
                }
            } else {
                {
                    let source_ml_source2_s22 = if (profile == "option result transpose") {
                        if (!inner && (owner == "std::option::Option")) {
                            self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(if (field == ((-1f64))) {
                                vec![String::from("child None"), String::from("result Ok")]
                            } else {
                                vec![String::from("match inner")]
                            })
                        } else {
                            if (inner && (owner == "std::result::Result")) {
                                self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(if (field == (0f64)) {
                                    vec![String::from("payload"), String::from("child Some"), String::from("result Ok")]
                                } else {
                                    vec![String::from("payload"), String::from("result Err")]
                                })
                            } else {
                                self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next
                    };
                    match source_ml_source2_s22.clone() {
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next => {
                            {
                                let source_ml_source2_s23 = if (profile == "result option transpose") {
                                    if (!inner && (owner == "std::result::Result")) {
                                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(if (field == (0f64)) {
                                            vec![String::from("match inner")]
                                        } else {
                                            vec![String::from("payload"), String::from("child Err"), String::from("result Some")]
                                        })
                                    } else {
                                        if (inner && (owner == "std::option::Option")) {
                                            self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(if (field == ((-1f64))) {
                                                vec![String::from("result None")]
                                            } else {
                                                vec![String::from("payload"), String::from("child Ok"), String::from("result Some")]
                                            })
                                        } else {
                                            self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next
                                };
                                match source_ml_source2_s23.clone() {
                                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next => {
                                        Vec::<String>::new()
                                    }
                                    self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(source_ml_source1_temporary_result7) => {
                                        source_ml_source1_temporary_result7
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6::MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(source_ml_source1_temporary_result7_2) => {
                            source_ml_source1_temporary_result7_2
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_nested_construction_position(term: String) -> f64 {
    if self::ml_array::includes(&vec![String::from("result None"), String::from("result Some"), String::from("result Ok"), String::from("result Err")], &term.clone(), 0f64, false, |a, b| a == b) {
        (-1f64)
    } else {
        if self::ml_array::includes(&vec![String::from("child None"), String::from("child Some"), String::from("child Ok"), String::from("child Err")], &term.clone(), 0f64, false, |a, b| a == b) {
            0f64
        } else {
            (-2f64)
        }
    }
}

pub fn read_rust_standard_sum_nested_construction_variant(term: String) -> String {
    if ((term == "result None") || (term == "child None")) {
        String::from("None")
    } else {
        if ((term == "result Some") || (term == "child Some")) {
            String::from("Some")
        } else {
            if ((term == "result Ok") || (term == "child Ok")) {
                String::from("Ok")
            } else {
                if ((term == "result Err") || (term == "child Err")) {
                    String::from("Err")
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_nested_source_version_error(owner: String, method: String, minor: f64) -> String {
    if ((self::read_rust_standard_sum_nested_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        if ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64))) {
            String::from("")
        } else {
            String::from("nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0")
        }
    }
}

pub fn read_rust_standard_sum_nested_source_version_range(owner: String, method: String) -> String {
    if ((self::read_rust_standard_sum_nested_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        String::from(">=1.90.0 <1.100.0")
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
                let source_ml_source2_comparison_operand24 = self::ml_array::optional_at(&direct_copies, self::ml_array::number_index(at));
                {
                    let source_ml_source2_comparison_operand25 = true;
                    !(match (source_ml_source2_comparison_operand24.clone(), source_ml_source2_comparison_operand25) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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
pub enum MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7 {
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Next,
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin7Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6 {
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Next,
    MlSource1TemporaryPlanRustStandardSumNestedBranchJoin6Return(Vec<String>),
}

fn ml_main() {
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_arity(String::from("std::option::Option"), String::from("flatten")), 0f64), "assertion 1");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_arity(String::from("std::option::Option"), String::from("transpose")), 0f64), "assertion 2");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_arity(String::from("std::result::Result"), String::from("transpose")), 0f64), "assertion 3");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_arity(String::from("std::result::Result"), String::from("flatten")), (-1f64)), "assertion 4");
    assert!(((self::read_rust_standard_sum_nested_profile(String::from("std::option::Option"), String::from("flatten"), String::from("std::option::Option"))) == "option flatten"), "assertion 5");
    assert!(((self::read_rust_standard_sum_nested_profile(String::from("std::option::Option"), String::from("transpose"), String::from("std::result::Result"))) == "option result transpose"), "assertion 6");
    assert!(((self::read_rust_standard_sum_nested_profile(String::from("std::result::Result"), String::from("transpose"), String::from("std::option::Option"))) == "result option transpose"), "assertion 7");
    assert!(((self::read_rust_standard_sum_nested_profile(String::from("std::option::Option"), String::from("transpose"), String::from("std::option::Option"))) == ""), "assertion 8");
    assert!(((self::read_rust_standard_sum_nested_profile(String::from("local::Option"), String::from("flatten"), String::from("std::option::Option"))) == ""), "assertion 9");
    assert!(((self::plan_rust_standard_sum_nested_result(String::from("option flatten"))) == (vec![String::from("inner")])), "assertion 10");
    assert!(((self::plan_rust_standard_sum_nested_result(String::from("option result transpose"))) == (vec![String::from("inner success"), String::from("std::option::Option"), String::from("inner error"), String::from("std::result::Result")])), "assertion 11");
    assert!(((self::plan_rust_standard_sum_nested_result(String::from("result option transpose"))) == (vec![String::from("inner success"), String::from("outer error"), String::from("std::result::Result"), String::from("std::option::Option")])), "assertion 12");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option flatten"), String::from("std::option::Option"), String::from("None"), false)) == (vec![String::from("result None")])), "assertion 13");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option flatten"), String::from("std::option::Option"), String::from("Some"), false)) == (vec![String::from("match inner")])), "assertion 14");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option flatten"), String::from("std::option::Option"), String::from("None"), true)) == (vec![String::from("result None")])), "assertion 15");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option flatten"), String::from("std::option::Option"), String::from("Some"), true)) == (vec![String::from("payload"), String::from("result Some")])), "assertion 16");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option result transpose"), String::from("std::option::Option"), String::from("None"), false)) == (vec![String::from("child None"), String::from("result Ok")])), "assertion 17");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option result transpose"), String::from("std::result::Result"), String::from("Ok"), true)) == (vec![String::from("payload"), String::from("child Some"), String::from("result Ok")])), "assertion 18");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("option result transpose"), String::from("std::result::Result"), String::from("Err"), true)) == (vec![String::from("payload"), String::from("result Err")])), "assertion 19");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("result option transpose"), String::from("std::result::Result"), String::from("Err"), false)) == (vec![String::from("payload"), String::from("child Err"), String::from("result Some")])), "assertion 20");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("result option transpose"), String::from("std::option::Option"), String::from("None"), true)) == (vec![String::from("result None")])), "assertion 21");
    assert!(((self::plan_rust_standard_sum_nested_branch(String::from("result option transpose"), String::from("std::option::Option"), String::from("Some"), true)) == (vec![String::from("payload"), String::from("child Ok"), String::from("result Some")])), "assertion 22");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_construction_position(String::from("result Some")), (-1f64)), "assertion 23");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_construction_position(String::from("child Err")), 0f64), "assertion 24");
    assert!(self::ml_number::same_value(self::read_rust_standard_sum_nested_construction_position(String::from("payload")), (-2f64)), "assertion 25");
    assert!(((self::read_rust_standard_sum_nested_construction_variant(String::from("child Some"))) == "Some"), "assertion 26");
    assert!(((self::read_rust_standard_sum_nested_source_version_range(String::from("std::option::Option"), String::from("flatten"))) == ">=1.90.0 <1.100.0"), "assertion 27");
    assert!(((self::read_rust_standard_sum_nested_source_version_error(String::from("std::option::Option"), String::from("transpose"), 90f64)) == ""), "assertion 28");
    assert!(((self::read_rust_standard_sum_nested_source_version_error(String::from("std::result::Result"), String::from("transpose"), 99f64)) == ""), "assertion 29");
    assert!(((self::read_rust_standard_sum_nested_source_version_error(String::from("std::option::Option"), String::from("flatten"), 89f64)) == "nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0"), "assertion 30");
    assert!(((self::read_rust_standard_sum_nested_source_version_error(String::from("std::result::Result"), String::from("transpose"), 100f64)) == "nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0"), "assertion 31");
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
fn all_literal_nested_sum_controls() { main(); }
