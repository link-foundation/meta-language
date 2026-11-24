pub fn read_rust_borrowed_source_read_error(state: String, loans: Vec<String>) -> String {
    {
        let source_ml_source1_s7 = if (state != "") {
            {
                let error = self::read_rust_owned_binding_read_error(state.clone());
                if !error.is_empty() {
                    self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(error.clone())
                } else {
                    self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next
        };
        match source_ml_source1_s7.clone() {
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next => {
                {
                    let source_ml_temporary_values8 = loans;
                    {
                        let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s8 = self::source_ml_temporary_read_rust_borrowed_source_read_error_loop10(source_ml_temporary_values8, source_ml_temporary_index8.clone());
                            match source_ml_source1_s8.clone() {
                                self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done => {
                                    String::from("")
                                }
                                self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(source_ml_temporary_result10) => {
                                    source_ml_temporary_result10.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(source_ml_temporary_result10_2) => {
                source_ml_temporary_result10_2.clone()
            }
        }
    }
}

pub fn consume_rust_borrowed_source_region(state: String) -> String {
    self::consume_rust_owned_binding_state(state.clone())
}

pub fn expire_rust_borrowed_source_region(state: String) -> String {
    self::expire_rust_owned_binding_state(state.clone())
}

pub fn read_rust_borrowed_source_adjustment(source: String, target: String, same: bool, inner: bool, source_inner: bool, place: bool, literal: bool, text: bool, source_references: f64, target_references: f64) -> String {
    if ((same && (source == target)) && (source_references == target_references)) {
        String::from("preserve")
    } else {
        if ((source == "inferred numeric") && (target == "copy")) {
            String::from("constrain primitive")
        } else {
            if ((target == "reference") && (same || inner)) {
                if ((source == "reference") && (target_references <= source_references)) {
                    String::from("project reference")
                } else {
                    if place {
                        String::from("borrow")
                    } else {
                        if literal {
                            String::from("static literal")
                        } else {
                            String::from("temporary borrow")
                        }
                    }
                }
            } else {
                if (((source == "reference") && (target == "copy")) && same) {
                    String::from("copy referent")
                } else {
                    if ((target == "box") && inner) {
                        String::from("construct box")
                    } else {
                        if ((source == "box") && source_inner) {
                            if (target == "copy") {
                                String::from("copy box payload")
                            } else {
                                String::from("move box payload")
                            }
                        } else {
                            if (((source == "reference") && (target == "string")) && text) {
                                String::from("materialize text")
                            } else {
                                String::from("unsupported")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_position_flow_error(role: String, same: bool, static_demand: bool, static_proof: bool) -> String {
    if !same {
        String::from("source expression qualifiers or owners disagree with their actual value flow")
    } else {
        if (static_demand && !static_proof) {
            String::from("a static source lifetime cannot be supplied by a local binder region")
        } else {
            if !(self::ml_array::includes(&vec![String::from("copy"), String::from("reference"), String::from("owned"), String::from("string"), String::from("box"), String::from("borrowed vector")], &role.clone(), 0f64, false, |a, b| a == b)) {
                String::from("the source expression needs a separate value-domain proof")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_borrowed_source_return_error(regions: Vec<f64>, allowed: Vec<f64>, static_regions: Vec<f64>, interface_known: bool) -> String {
    if !interface_known {
        String::from("a borrowed source result needs its actual source parameter lifetime interface")
    } else {
        {
            let source_ml_temporary_values11 = regions;
            {
                let source_ml_temporary_index11 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_read_rust_borrowed_source_return_error_loop12(allowed, static_regions, source_ml_temporary_values11, source_ml_temporary_index11.clone());
                    match source_ml_source1_s9.clone() {
                        self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done => {
                            String::from("")
                        }
                        self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(source_ml_temporary_result12) => {
                            source_ml_temporary_result12.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_lifetime_error(demand: String, sources: Vec<String>) -> String {
    if ((demand == "") || (demand == "_")) {
        String::from("")
    } else {
        {
            let source_ml_temporary_values13 = sources;
            {
                let source_ml_temporary_index13 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s10 = self::source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14(demand.clone(), source_ml_temporary_values13, source_ml_temporary_index13.clone());
                    match source_ml_source1_s10.clone() {
                        self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done => {
                            String::from("")
                        }
                        self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(source_ml_temporary_result14) => {
                            source_ml_temporary_result14.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_target_field_error(kind: String, wrappers: Vec<String>, child: bool) -> String {
    if ((self::ml_array::includes(&wrappers, &String::from("reference"), 0f64, false, |a, b| a == b) || child) || self::ml_array::includes(&vec![String::from("function"), String::from("tuple"), String::from("record"), String::from("map"), String::from("set"), String::from("cell")], &kind.clone(), 0f64, false, |a, b| a == b)) {
        String::from("a borrowed referent needs a closed target-copy field graph with represented source qualifiers")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_source_operation(kind: String) -> String {
    if (kind == "toString") {
        String::from("show")
    } else {
        if self::ml_array::includes(&vec![String::from("lit"), String::from("unit"), String::from("var"), String::from("let"), String::from("if"), String::from("array"), String::from("call"), String::from("ctor"), String::from("index"), String::from("length"), String::from("print"), String::from("cast"), String::from("unary"), String::from("binary"), String::from("math"), String::from("show"), String::from("concat")], &kind.clone(), 0f64, false, |a, b| a == b) {
            kind.clone()
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_source_region_budget_error(visits: f64, depth: f64, roots: f64, loans: f64) -> String {
    {
        let source_ml_temporary_values15 = vec![visits, depth, roots, loans];
        {
            let source_ml_temporary_index15 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s11 = self::source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16(source_ml_temporary_values15, source_ml_temporary_index15.clone());
                match source_ml_source1_s11.clone() {
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done => {
                        if ((((visits > (100000f64)) || (depth > (256f64))) || (roots > (100000f64))) || (loans > (1048576f64))) {
                            String::from("the immutable source binder and region proof exceeds its finite resource profile")
                        } else {
                            String::from("")
                        }
                    }
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(source_ml_temporary_result16) => {
                        source_ml_temporary_result16.clone()
                    }
                }
            }
        }
    }
}

pub fn reserve_rust_borrowed_source_position(nodes: f64, units: f64, add_nodes: f64, add_units: f64) -> Vec<f64> {
    {
        let source_ml_temporary_values17 = vec![nodes, units, add_nodes, add_units];
        {
            let source_ml_temporary_index17 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s12 = self::source_ml_temporary_reserve_rust_borrowed_source_position_loop18(source_ml_temporary_values17, source_ml_temporary_index17.clone());
                match source_ml_source1_s12.clone() {
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Done => {
                        {
                            let next_nodes = (nodes + (4f64 * add_nodes));
                            {
                                let next_units = (units + (4f64 * add_units));
                                if ((next_nodes > (1048576f64)) || (next_units > (8388608f64))) {
                                    Vec::<f64>::new()
                                } else {
                                    vec![next_nodes, next_units]
                                }
                            }
                        }
                    }
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(source_ml_temporary_result18) => {
                        source_ml_temporary_result18
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_runtime_error(selected: bool, runtime_shell: bool) -> String {
    if (selected && runtime_shell) {
        String::from("borrowed source body positions require their independent runtime-shell binder adapter")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_source_observation_error(role: String, operation: String) -> String {
    if self::ml_array::includes(&vec![String::from("source state packet"), String::from("source loop packet"), String::from("source loop control")], &role.clone(), 0f64, false, |a, b| a == b) {
        String::from("source state packets are interpreted through their actual result/state slots")
    } else {
        if (self::ml_array::includes(&vec![String::from("reference"), String::from("box"), String::from("owned"), String::from("borrowed vector")], &role.clone(), 0f64, false, |a, b| a == b) && (operation != "len")) {
            String::from("this source observation needs an independently represented value or source trait proof")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_source_observation_role(role: String, kind: String, owner: String, path: String, argument_count: f64, wrappers: Vec<String>) -> String {
    if ((((((role == "reference") && (kind == "string")) && (((owner == "primitive") && (path == "str")) || ((owner == "library") && self::ml_array::includes(&vec![String::from("std::string::String"), String::from("alloc::string::String")], &path.clone(), 0f64, false, |a, b| a == b)))) && (argument_count == (0f64))) && (((wrappers.len() as f64)) == (1f64))) && {
        let source_ml_source1_comparison_operand13 = self::ml_array::optional_at(&wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand14 = String::from("reference");
            match (source_ml_source1_comparison_operand13.clone(), source_ml_source1_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) {
        String::from("text")
    } else {
        role.clone()
    }
}

pub fn read_rust_borrowed_source_match_error(role: String, kind: String, pattern: String, fields: f64, bindings: f64) -> String {
    if ((role != "reference") || (kind != "data")) {
        String::from("source matching requires its actual borrowed nominal place adapter")
    } else {
        if !(self::ml_array::includes(&vec![String::from("wild"), String::from("bind"), String::from("ctor")], &pattern.clone(), 0f64, false, |a, b| a == b)) {
            String::from("this source pattern needs its independent binding projection")
        } else {
            if ((pattern == "ctor") && ((!(self::ml_math::is_safe_integer(fields)) || (fields < (0f64))) || (fields != bindings))) {
                String::from("source constructor patterns need every actual declared field position")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_borrowed_source_owned_match_error(composed: bool, kind: String, pattern: String, fields: f64, bindings: f64) -> String {
    if !composed {
        String::from("owned matches require the explicit closed-call region composition")
    } else {
        self::read_rust_borrowed_source_match_error(String::from("reference"), kind.clone(), pattern.clone(), fields, bindings)
    }
}

pub fn reserve_rust_borrowed_source_branches(cells: f64, addition: f64) -> f64 {
    if ((((((!(self::ml_math::is_safe_integer(cells)) || !(self::ml_math::is_safe_integer(addition))) || (cells < (0f64))) || (addition < (0f64))) || {
        let source_ml_source1_same_value_operand15 = cells;
        {
            let source_ml_source1_same_value_operand16 = (-0f64);
            (((source_ml_source1_same_value_operand15 == source_ml_source1_same_value_operand16) && ((source_ml_source1_same_value_operand15 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand15)) == ((1f64 / source_ml_source1_same_value_operand16))))) || ((source_ml_source1_same_value_operand15 != source_ml_source1_same_value_operand15) && (source_ml_source1_same_value_operand16 != source_ml_source1_same_value_operand16)))
        }
    }) || {
        let source_ml_source1_same_value_operand17 = addition;
        {
            let source_ml_source1_same_value_operand18 = (-0f64);
            (((source_ml_source1_same_value_operand17 == source_ml_source1_same_value_operand18) && ((source_ml_source1_same_value_operand17 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand17)) == ((1f64 / source_ml_source1_same_value_operand18))))) || ((source_ml_source1_same_value_operand17 != source_ml_source1_same_value_operand17) && (source_ml_source1_same_value_operand18 != source_ml_source1_same_value_operand18)))
        }
    }) || (addition > ((1048576f64 - cells)))) {
        (-1f64)
    } else {
        (cells + addition)
    }
}

pub fn merge_rust_borrowed_source_region(left: String, right: String) -> String {
    self::merge_rust_owned_binding_states(left.clone(), right.clone(), true, true)
}

pub fn read_rust_borrowed_source_match_operation(kind: String) -> String {
    if (kind == "match") {
        String::from("match")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_source_box_reference(source: Vec<String>, target: Vec<String>, canonical: bool, same_leaf: bool) -> bool {
    if ((((!canonical || !same_leaf) || {
        let source_ml_source1_comparison_operand19 = self::ml_array::optional_at(&source, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand20 = String::from("reference");
            !(match (source_ml_source1_comparison_operand19.clone(), source_ml_source1_comparison_operand20.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || {
        let source_ml_source1_comparison_operand21 = self::ml_array::optional_at(&target, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand22 = String::from("reference");
            !(match (source_ml_source1_comparison_operand21.clone(), source_ml_source1_comparison_operand22.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || (((source.len() as f64)) <= ((target.len() as f64)))) {
        false
    } else {
        {
            let index = 1f64;
            {
                let source_ml_source1_s23 = self::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19(source, index);
                match source_ml_source1_s23.clone() {
                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done => {
                        {
                            let index_2 = 1f64;
                            {
                                let source_ml_source1_s24 = self::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20(target, index_2);
                                match source_ml_source1_s24.clone() {
                                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done => {
                                        true
                                    }
                                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(source_ml_temporary_result20) => {
                                        source_ml_temporary_result20
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(source_ml_temporary_result20_2) => {
                        source_ml_temporary_result20_2
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_iterator_operation(kind: String, operation: String) -> String {
    if (kind == "append") {
        String::from("iterator concat")
    } else {
        if ((kind == "method") && self::ml_array::includes(&vec![String::from("map"), String::from("slice")], &operation.clone(), 0f64, false, |a, b| a == b)) {
            format!("{}{}", (String::from("iterator ")), operation)
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_source_storage_write_error(state: String) -> String {
    if self::ml_array::includes(&vec![String::from("available"), String::from("moved"), String::from("possibly moved")], &state.clone(), 0f64, false, |a, b| a == b) {
        String::from("")
    } else {
        String::from("a source whole-root write requires its initialized lexical storage scope")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Done,
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(bool),
}

pub fn source_ml_temporary_accept_rust_owned_binding_guard_loop1(mut before: Vec<String>, mut after: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    loop {
        return if (index < ((before.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand25 = self::ml_array::optional_at(&before, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand26 = self::ml_array::optional_at(&after, self::ml_array::number_index(index));
                    !(match (source_ml_source1_comparison_operand25.clone(), source_ml_source1_comparison_operand26.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Done
        };
    }
}

pub fn source_ml_temporary_merge_rust_owned_loan_bindings_loop4(mut result: Vec<String>, mut source_ml_temporary_values2: Vec<String>, mut source_ml_temporary_index2: self::ml::Big) -> Vec<String> {
    loop {
        return if (source_ml_temporary_index2 < (self::ml::Big::from_u128(source_ml_temporary_values2.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_temporary_values2, source_ml_temporary_index2.to_index());
                if !(self::ml_array::includes(&result, &identity.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![identity.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let source_ml_temporary_index2_2 = source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index2) = (result_2, source_ml_temporary_index2_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index2_3 = source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index2 = source_ml_temporary_index2_3.clone();
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

pub fn source_ml_temporary_merge_rust_owned_loan_bindings_loop5(mut result: Vec<String>, mut source_ml_temporary_values3: Vec<String>, mut source_ml_temporary_index3: self::ml::Big) -> Vec<String> {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                if !(self::ml_array::includes(&result, &identity.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![identity.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index3) = (result_2, source_ml_temporary_index3_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index3_3 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index3 = source_ml_temporary_index3_3.clone();
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    MlTemporaryReadRustOwnedLoanReadErrorLoop7Done,
    MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(String),
}

pub fn source_ml_temporary_read_rust_owned_loan_read_error_loop7(mut source_ml_temporary_values6: Vec<String>, mut source_ml_temporary_index6: self::ml::Big) -> self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    loop {
        return if (source_ml_temporary_index6 < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))) {
            {
                let state = self::ml_array::at(&source_ml_temporary_values6, source_ml_temporary_index6.to_index());
                if (state != "available") {
                    self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(self::read_rust_owned_binding_read_error(String::from("")))
                } else {
                    {
                        let source_ml_temporary_index6_2 = source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index6 = source_ml_temporary_index6_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result {
    MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done,
    MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_read_error_loop10(mut source_ml_temporary_values8: Vec<String>, mut source_ml_temporary_index8: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result {
    loop {
        return if (source_ml_temporary_index8 < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))) {
            {
                let loan = self::ml_array::at(&source_ml_temporary_values8, source_ml_temporary_index8.to_index());
                {
                    let error = self::read_rust_owned_binding_read_error(loan.clone());
                    if !error.is_empty() {
                        self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(String::from("a source expression uses a region whose actual binder has moved or expired"))
                    } else {
                        {
                            let source_ml_temporary_index8_2 = source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index8 = source_ml_temporary_index8_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReadErrorJoin9 {
    MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next,
    MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result {
    MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done,
    MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_return_error_loop12(mut allowed: Vec<f64>, mut static_regions: Vec<f64>, mut source_ml_temporary_values11: Vec<f64>, mut source_ml_temporary_index11: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result {
    loop {
        return if (source_ml_temporary_index11 < (self::ml::Big::from_u128(source_ml_temporary_values11.len() as u128))) {
            {
                let region = self::ml_array::at(&source_ml_temporary_values11, source_ml_temporary_index11.to_index());
                if (!(self::ml_array::includes(&allowed, &region, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) && !(self::ml_array::includes(&static_regions, &region, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(String::from("a returned source region is not derived from its actual enclosing parameter interface"))
                } else {
                    {
                        let source_ml_temporary_index11_2 = source_ml_temporary_index11.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index11 = source_ml_temporary_index11_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result {
    MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done,
    MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14(mut demand: String, mut source_ml_temporary_values13: Vec<String>, mut source_ml_temporary_index13: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result {
    loop {
        return if (source_ml_temporary_index13 < (self::ml::Big::from_u128(source_ml_temporary_values13.len() as u128))) {
            {
                let source = self::ml_array::at(&source_ml_temporary_values13, source_ml_temporary_index13.to_index());
                if ((source != "static") && (source != demand)) {
                    self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(String::from("an expression lifetime is not supplied by its actual source binder regions"))
                } else {
                    {
                        let source_ml_temporary_index13_2 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index13 = source_ml_temporary_index13_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result {
    MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done,
    MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16(mut source_ml_temporary_values15: Vec<f64>, mut source_ml_temporary_index15: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result {
    loop {
        return if (source_ml_temporary_index15 < (self::ml::Big::from_u128(source_ml_temporary_values15.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values15, source_ml_temporary_index15.to_index());
                if ((!(self::ml_math::is_safe_integer(value)) || (value < (0f64))) || {
                    let source_ml_source1_same_value_operand27 = value;
                    {
                        let source_ml_source1_same_value_operand28 = (-0f64);
                        (((source_ml_source1_same_value_operand27 == source_ml_source1_same_value_operand28) && ((source_ml_source1_same_value_operand27 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand27)) == ((1f64 / source_ml_source1_same_value_operand28))))) || ((source_ml_source1_same_value_operand27 != source_ml_source1_same_value_operand27) && (source_ml_source1_same_value_operand28 != source_ml_source1_same_value_operand28)))
                    }
                }) {
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(String::from("invalid source region proof resource count"))
                } else {
                    {
                        let source_ml_temporary_index15_2 = source_ml_temporary_index15.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index15 = source_ml_temporary_index15_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReserveRustBorrowedSourcePositionLoop18Result {
    MlTemporaryReserveRustBorrowedSourcePositionLoop18Done,
    MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(Vec<f64>),
}

pub fn source_ml_temporary_reserve_rust_borrowed_source_position_loop18(mut source_ml_temporary_values17: Vec<f64>, mut source_ml_temporary_index17: self::ml::Big) -> self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result {
    loop {
        return if (source_ml_temporary_index17 < (self::ml::Big::from_u128(source_ml_temporary_values17.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values17, source_ml_temporary_index17.to_index());
                if ((!(self::ml_math::is_safe_integer(value)) || (value < (0f64))) || {
                    let source_ml_source1_same_value_operand29 = value;
                    {
                        let source_ml_source1_same_value_operand30 = (-0f64);
                        (((source_ml_source1_same_value_operand29 == source_ml_source1_same_value_operand30) && ((source_ml_source1_same_value_operand29 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand29)) == ((1f64 / source_ml_source1_same_value_operand30))))) || ((source_ml_source1_same_value_operand29 != source_ml_source1_same_value_operand29) && (source_ml_source1_same_value_operand30 != source_ml_source1_same_value_operand30)))
                    }
                }) {
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(Vec::<f64>::new())
                } else {
                    {
                        let source_ml_temporary_index17_2 = source_ml_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index17 = source_ml_temporary_index17_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result {
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done,
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(bool),
}

pub fn source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20(mut target: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result {
    loop {
        return if (index < ((target.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand31 = self::ml_array::optional_at(&target, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand32 = String::from("box");
                    !(match (source_ml_source1_comparison_operand31.clone(), source_ml_source1_comparison_operand32.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result {
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done,
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(bool),
}

pub fn source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19(mut source: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result {
    loop {
        return if (index < ((source.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand33 = self::ml_array::optional_at(&source, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand34 = String::from("box");
                    !(match (source_ml_source1_comparison_operand33.clone(), source_ml_source1_comparison_operand34.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_rust_borrowed_source_value_role(String::from("data"), String::from(""), false, String::from("declaration"), String::from("Node"), 0f64, vec![String::from("reference")], false)) == "reference"), "assertion 1");
    assert!(((self::read_rust_borrowed_source_value_role(String::from("array"), String::from(""), false, String::from("library"), String::from("std::vec::Vec"), 1f64, Vec::<String>::new(), true)) == "borrowed vector"), "assertion 2");
    assert!(((self::read_rust_borrowed_source_value_role(String::from("array"), String::from(""), false, String::from("declaration"), String::from("std::vec::Vec"), 1f64, Vec::<String>::new(), true)) == ""), "assertion 3");
    assert!(((self::read_rust_borrowed_source_use_plan(String::from("reference"), String::from("move"))) == "read"), "assertion 4");
    assert!(((self::read_rust_borrowed_source_use_plan(String::from("borrowed vector"), String::from("move"))) == "consume"), "assertion 5");
    assert!(((self::read_rust_borrowed_source_read_error(String::from("available"), vec![String::from("available")])) == ""), "assertion 6");
    assert!(((self::read_rust_borrowed_source_read_error(String::from("available"), vec![String::from("moved")])) == "a source expression uses a region whose actual binder has moved or expired"), "assertion 7");
    assert!(((self::read_rust_borrowed_source_adjustment(String::from("owned"), String::from("reference"), true, true, false, true, false, false, 0f64, 1f64)) == "borrow"), "assertion 8");
    assert!(((self::read_rust_borrowed_source_adjustment(String::from("reference"), String::from("reference"), true, false, false, true, false, false, 1f64, 1f64)) == "preserve"), "assertion 9");
    assert!(((self::read_rust_borrowed_source_return_error(vec![2f64], vec![2f64], vec![0f64], true)) == ""), "assertion 10");
    assert!(((self::read_rust_borrowed_source_return_error(vec![3f64], vec![2f64], vec![0f64], true)) == "a returned source region is not derived from its actual enclosing parameter interface"), "assertion 11");
    assert!(((self::read_rust_borrowed_source_return_error(vec![0f64], Vec::<f64>::new(), vec![0f64], true)) == ""), "assertion 12");
    assert!(((self::read_rust_borrowed_source_lifetime_error(String::from("a"), vec![String::from("a"), String::from("static")])) == ""), "assertion 13");
    assert!(((self::read_rust_borrowed_source_lifetime_error(String::from("static"), vec![String::from("a")])) == "an expression lifetime is not supplied by its actual source binder regions"), "assertion 14");
    assert!(((self::read_rust_borrowed_source_target_field_error(String::from("data"), vec![String::from("reference")], false)) == "a borrowed referent needs a closed target-copy field graph with represented source qualifiers"), "assertion 15");
    assert!(((self::read_rust_borrowed_source_operation(String::from("match"))) == ""), "assertion 16");
    assert!(((self::read_rust_borrowed_source_operation(String::from("if"))) == "if"), "assertion 17");
    assert!(((self::read_rust_borrowed_source_region_budget_error(100000f64, 256f64, 100000f64, 1048576f64)) == ""), "assertion 18");
    assert!(((self::read_rust_borrowed_source_region_budget_error(100001f64, 0f64, 1f64, 0f64)) == "the immutable source binder and region proof exceeds its finite resource profile"), "assertion 19");
    assert!(((self::read_rust_borrowed_source_region_budget_error(f64::NAN, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 20");
    assert!(((self::read_rust_borrowed_source_region_budget_error(f64::INFINITY, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 21");
    assert!(((self::read_rust_borrowed_source_region_budget_error((-0f64), 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 22");
    assert!(((self::read_rust_borrowed_source_region_budget_error((-1f64), 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 23");
    assert!(((self::read_rust_borrowed_source_region_budget_error(0.5f64, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 24");
    assert!(((self::read_rust_borrowed_source_region_budget_error(9007199254740992f64, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 25");
    assert!(((self::reserve_rust_borrowed_source_position(0f64, 0f64, 1f64, 2f64)) == (vec![4f64, 8f64])), "assertion 26");
    assert!(((self::reserve_rust_borrowed_source_position(1048572f64, 8388600f64, 1f64, 2f64)) == (vec![1048576f64, 8388608f64])), "assertion 27");
    assert!(((self::reserve_rust_borrowed_source_position(1048573f64, 0f64, 1f64, 0f64)) == (Vec::<f64>::new())), "assertion 28");
    assert!(((self::reserve_rust_borrowed_source_position(0f64, 8388601f64, 0f64, 2f64)) == (Vec::<f64>::new())), "assertion 29");
    assert!(((self::reserve_rust_borrowed_source_position(0f64, 0f64, (-0f64), 0f64)) == (Vec::<f64>::new())), "assertion 30");
    assert!(((self::read_rust_borrowed_source_runtime_error(true, true)) == "borrowed source body positions require their independent runtime-shell binder adapter"), "assertion 31");
    assert!(((self::read_rust_borrowed_source_runtime_error(false, true)) == ""), "assertion 32");
    assert!(((self::read_rust_borrowed_source_runtime_error(true, false)) == ""), "assertion 33");
    assert!(((self::read_rust_borrowed_source_operation(String::from("toString"))) == "show"), "assertion 34");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("reference"), String::from("show"))) == "this source observation needs an independently represented value or source trait proof"), "assertion 35");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("owned"), String::from("display"))) == "this source observation needs an independently represented value or source trait proof"), "assertion 36");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("copy"), String::from("binary"))) == ""), "assertion 37");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("borrowed vector"), String::from("len"))) == ""), "assertion 38");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("primitive"), String::from("str"), 0f64, vec![String::from("reference")])) == "text"), "assertion 39");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("library"), String::from("std::string::String"), 0f64, vec![String::from("reference")])) == "text"), "assertion 40");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("library"), String::from("alloc::string::String"), 0f64, vec![String::from("reference")])) == "text"), "assertion 41");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("declaration"), String::from("str"), 0f64, vec![String::from("reference")])) == "reference"), "assertion 42");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("primitive"), String::from("str"), 0f64, vec![String::from("reference"), String::from("reference")])) == "reference"), "assertion 43");
    assert!(((self::read_rust_borrowed_source_match_operation(String::from("match"))) == "match"), "assertion 44");
    assert!(((self::read_rust_borrowed_source_match_operation(String::from("if"))) == ""), "assertion 45");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("reference"), String::from("data"), String::from("ctor"), 2f64, 2f64)) == ""), "assertion 46");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("owned"), String::from("data"), String::from("ctor"), 2f64, 2f64)) == "source matching requires its actual borrowed nominal place adapter"), "assertion 47");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("reference"), String::from("data"), String::from("ctor"), 2f64, 1f64)) == "source constructor patterns need every actual declared field position"), "assertion 48");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("reference"), String::from("data"), String::from("wild"), 0f64, 0f64)) == ""), "assertion 49");
    assert!(self::ml_number::same_value(self::reserve_rust_borrowed_source_branches(1048570f64, 6f64), 1048576f64), "assertion 50");
    assert!(self::ml_number::same_value(self::reserve_rust_borrowed_source_branches(1048570f64, 7f64), (-1f64)), "assertion 51");
    assert!(self::ml_number::same_value(self::reserve_rust_borrowed_source_branches(0f64, (-0f64)), (-1f64)), "assertion 52");
    assert!(((self::merge_rust_borrowed_source_region(String::from("available"), String::from("moved"))) == "possibly moved"), "assertion 53");
    assert!(((self::merge_rust_borrowed_source_region(String::from("available"), String::from("available"))) == "available"), "assertion 54");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("box")], vec![String::from("reference")], true, true)) == (true)), "assertion 55");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("box")], vec![String::from("reference")], false, true)) == (false)), "assertion 56");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("box")], vec![String::from("reference")], true, false)) == (false)), "assertion 57");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("reference")], vec![String::from("reference")], true, true)) == (false)), "assertion 58");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference")], vec![String::from("reference"), String::from("box")], true, true)) == (false)), "assertion 59");
    assert!(((self::read_rust_borrowed_source_adjustment(String::from("reference"), String::from("reference"), false, false, false, true, false, false, 1f64, 1f64)) == "unsupported"), "assertion 60");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("map"))) == "iterator map"), "assertion 61");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("slice"))) == "iterator slice"), "assertion 62");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("append"), String::from(""))) == "iterator concat"), "assertion 63");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("concat"))) == ""), "assertion 64");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("reduce"))) == ""), "assertion 65");
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
fn all_literal_borrowed_source_region_controls() { main(); }
