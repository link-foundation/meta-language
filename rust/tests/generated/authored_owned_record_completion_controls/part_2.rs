pub fn read_rust_ordinary_match_completion_presence(kinds: Vec<String>, values: Vec<String>) -> f64 {
    if (((((kinds.len() as f64)) != ((values.len() as f64))) || {
        let source_ml_source1_comparison_operand6 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand7 = String::from("match");
            !(match (source_ml_source1_comparison_operand6.clone(), source_ml_source1_comparison_operand7.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || {
        let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand9 = String::from("identifier");
            !(match (source_ml_source1_comparison_operand8.clone(), source_ml_source1_comparison_operand9.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let opened = false;
                {
                    let selected = false;
                    {
                        let index = 1f64;
                        {
                            let source_ml_source1_s10 = self::source_ml_temporary_read_rust_ordinary_match_completion_presence_loop5(kinds, values, depth, opened, selected, index);
                            match source_ml_source1_s10.clone() {
                                self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Done => {
                                    (-1f64)
                                }
                                self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Return(source_ml_temporary_result5) => {
                                    source_ml_temporary_result5
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_match_arm_count_error(count: f64) -> String {
    if (((count >= (1f64)) && (count <= (256f64))) && (((count % 1f64)) == (0f64))) {
        String::from("")
    } else {
        String::from("the complete ordinary match arms exceed their source budget")
    }
}

pub fn test_rust_ordinary_completion_transferred_result(transfers: Vec<bool>) -> bool {
    if (((transfers.len() as f64)) == (0f64)) {
        false
    } else {
        {
            let source_ml_temporary_values7 = transfers;
            {
                let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s11 = self::source_ml_temporary_test_rust_ordinary_completion_transferred_result_loop8(source_ml_temporary_values7, source_ml_temporary_index7.clone());
                    match source_ml_source1_s11.clone() {
                        self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Done => {
                            true
                        }
                        self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Return(source_ml_temporary_result8) => {
                            source_ml_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_completion_snapshot_error(entries: f64, added_entries: f64, text: f64, added_text: f64) -> String {
    if ((((((((((((entries % 1f64)) != (0f64)) || (((added_entries % 1f64)) != (0f64))) || (((text % 1f64)) != (0f64))) || (((added_text % 1f64)) != (0f64))) || (entries < (0f64))) || (added_entries < (0f64))) || (text < (0f64))) || (added_text < (0f64))) || (entries > ((100000f64 - added_entries)))) || (text > ((16777216f64 - added_text)))) {
        String::from("the actual completion exit snapshots exceed their finite source budget")
    } else {
        String::from("")
    }
}

pub fn read_rust_ordinary_completion_sequence_kind(first: String, second: String) -> String {
    if (first == "value") {
        second.clone()
    } else {
        if (((first == "broken") || (first == "continued")) || (first == "completed loop alternatives")) {
            first.clone()
        } else {
            if (((second == "broken") || (second == "continued")) || (second == "completed loop alternatives")) {
                String::from("completed loop alternatives")
            } else {
                if ((first == "returned") || (first == "aborted")) {
                    first.clone()
                } else {
                    if ((second == "returned") || (second == "aborted")) {
                        String::from("returned")
                    } else {
                        String::from("mixed")
                    }
                }
            }
        }
    }
}

pub fn read_rust_function_completion_presence(values: Vec<String>) -> f64 {
    if {
        let source_ml_source1_comparison_operand12 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand13 = String::from("{");
            !(match (source_ml_source1_comparison_operand12.clone(), source_ml_source1_comparison_operand13.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    } {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let index = 0f64;
                {
                    let source_ml_source1_s14 = self::source_ml_temporary_read_rust_function_completion_presence_loop9(values, depth, index);
                    match source_ml_source1_s14.clone() {
                        self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Done => {
                            (-1f64)
                        }
                        self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(source_ml_temporary_result9) => {
                            source_ml_temporary_result9
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_ordinary_statement_disposition(kind: String, collecting: bool, proven: bool, type_: String, indirected: bool) -> String {
    {
        let original = self::read_rust_unit_statement_disposition(kind.clone(), collecting, proven, type_.clone(), indirected);
        if (original != "refuse") {
            original.clone()
        } else {
            if (!proven || indirected) {
                String::from("refuse")
            } else {
                if ((((((type_ == "bool") || (type_ == "fixed")) || (type_ == "float")) || (type_ == "int")) || (type_ == "nat")) || (type_ == "string")) {
                    String::from("bind")
                } else {
                    String::from("refuse")
                }
            }
        }
    }
}

pub fn read_rust_implicit_block_statement_disposition(kind: String, block: bool, continuation: bool, collecting: bool, proven: bool, type_: String, indirected: bool) -> String {
    if (!continuation || !((((block || (kind == "if")) || (kind == "match")) || (kind == "match1")))) {
        String::from("")
    } else {
        if indirected {
            String::from("refuse")
        } else {
            if (collecting || (proven && (type_ == "unit"))) {
                String::from("bind")
            } else {
                String::from("refuse")
            }
        }
    }
}

pub fn accept_rust_block_expression_statement_boundary(opening: String, closing: String) -> bool {
    (self::ml_array::includes(&vec![String::from("if"), String::from("match"), String::from("{")], &opening.clone(), 0f64, false, |a, b| a == b) && (closing == "}"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result {
    MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done,
    MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(bool),
}

pub fn source_ml_temporary_accept_rust_ordinary_completion_profiles_loop1(mut profiles: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result {
    loop {
        return if (index < ((profiles.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand15 = self::ml_array::optional_at(&profiles, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand16 = String::from("");
                    !(match (source_ml_source1_comparison_operand15.clone(), source_ml_source1_comparison_operand16.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(false)
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
            self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionReturnPresenceLoop2Result {
    MlTemporaryReadRustFunctionReturnPresenceLoop2Done,
    MlTemporaryReadRustFunctionReturnPresenceLoop2Return(f64),
}

pub fn source_ml_temporary_read_rust_function_return_presence_loop2(mut values: Vec<String>, mut depth: f64, mut index: f64) -> self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| String::from(""));
                if (value == "return") {
                    self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(1f64)
                } else {
                    {
                        let depth_3 = if (value == "{") {
                            {
                                let depth_2 = (depth + 1f64);
                                depth_2
                            }
                        } else {
                            depth
                        };
                        if (value == "}") {
                            {
                                let depth_4 = (depth_3 - 1f64);
                                if (depth_4 == (0f64)) {
                                    self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(0f64)
                                } else {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            (depth, index) = (depth_4, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    MlTemporaryFindRustTerminalReturnBoundaryLoop3Done,
    MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(f64),
}

pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop4(mut values: Vec<String>, mut end: f64) -> f64 {
    loop {
        return if {
            let source_ml_source1_comparison_operand17 = self::ml_array::optional_at(&values, self::ml_array::number_index(end));
            {
                let source_ml_source1_comparison_operand18 = String::from(";");
                match (source_ml_source1_comparison_operand17.clone(), source_ml_source1_comparison_operand18.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } {
            {
                let end_2 = (end + 1f64);
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop3(mut values: Vec<String>, mut stack: Vec<String>, mut at: f64) -> self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| String::from(""));
                if (((value == "(") || (value == "[")) || (value == "{")) {
                    {
                        let stack_2 = { let (ml_append_right, ml_append_left) = (vec![value.clone()], stack); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (stack, at) = (stack_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if ((((stack.len() as f64)) == (0f64)) && ((value == ";") || (value == "}"))) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_temporary_find_rust_terminal_return_boundary_loop4(values.clone(), end);
                                if (end_2 >= ((values.len() as f64))) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return((-2f64))
                                } else {
                                    if {
                                        let source_ml_source1_comparison_operand19 = self::ml_array::optional_at(&values, self::ml_array::number_index(end_2));
                                        {
                                            let source_ml_source1_comparison_operand20 = String::from("}");
                                            match (source_ml_source1_comparison_operand19.clone(), source_ml_source1_comparison_operand20.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    } {
                                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(end_2)
                                    } else {
                                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return((-1f64))
                                    }
                                }
                            }
                        }
                    } else {
                        if (((value == ")") || (value == "]")) || (value == "}")) {
                            {
                                let previous = (self::ml_array::optional_at(&stack, self::ml_array::number_index(((stack.len() as f64) - 1f64)))).clone().unwrap_or_else(|| String::from(""));
                                if ((((value == ")") && (previous != "(")) || ((value == "]") && (previous != "["))) || ((value == "}") && (previous != "{"))) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return((-1f64))
                                } else {
                                    {
                                        let stack_3 = self::ml_array::slice(&stack, 0f64, (-1f64));
                                        {
                                            let at_3 = (at + 1f64);
                                            {
                                                (stack, at) = (stack_3, at_3);
                                                continue;
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
            self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result {
    MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Done,
    MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6 {
    MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(f64, bool),
}

pub fn source_ml_temporary_read_rust_ordinary_match_completion_presence_loop5(mut kinds: Vec<String>, mut values: Vec<String>, mut depth: f64, mut opened: bool, mut selected: bool, mut index: f64) -> self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let source_ml_source1_s25 = if ({
                    let source_ml_source1_comparison_operand26 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand27 = String::from("punct");
                        match (source_ml_source1_comparison_operand26.clone(), source_ml_source1_comparison_operand27.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                } && {
                    let source_ml_source1_comparison_operand28 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand29 = String::from("{");
                        match (source_ml_source1_comparison_operand28.clone(), source_ml_source1_comparison_operand29.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                }) {
                    {
                        let depth_2 = (depth + 1f64);
                        {
                            let opened_2 = true;
                            self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(depth_2, opened_2)
                        }
                    }
                } else {
                    self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(depth, opened)
                };
                match source_ml_source1_s25.clone() {
                    self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6::MlTemporaryReadRustOrdinaryMatchCompletionPresenceJoin6Next(depth_3, opened_3) => {
                        {
                            let selected_3 = if (opened_3 && (({
                                let source_ml_source1_comparison_operand30 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand31 = String::from("identifier");
                                    match (source_ml_source1_comparison_operand30.clone(), source_ml_source1_comparison_operand31.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } && (({
                                let source_ml_source1_comparison_operand32 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand33 = String::from("return");
                                    match (source_ml_source1_comparison_operand32.clone(), source_ml_source1_comparison_operand33.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } || {
                                let source_ml_source1_comparison_operand34 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand35 = String::from("break");
                                    match (source_ml_source1_comparison_operand34.clone(), source_ml_source1_comparison_operand35.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            }) || {
                                let source_ml_source1_comparison_operand36 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand37 = String::from("continue");
                                    match (source_ml_source1_comparison_operand36.clone(), source_ml_source1_comparison_operand37.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            })) || ({
                                let source_ml_source1_comparison_operand38 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand39 = String::from("punct");
                                    match (source_ml_source1_comparison_operand38.clone(), source_ml_source1_comparison_operand39.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } && {
                                let source_ml_source1_comparison_operand40 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand41 = String::from("?");
                                    match (source_ml_source1_comparison_operand40.clone(), source_ml_source1_comparison_operand41.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            }))) {
                                {
                                    let selected_2 = true;
                                    selected_2
                                }
                            } else {
                                selected
                            };
                            if (({
                                let source_ml_source1_comparison_operand42 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand43 = String::from("punct");
                                    match (source_ml_source1_comparison_operand42.clone(), source_ml_source1_comparison_operand43.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } && {
                                let source_ml_source1_comparison_operand44 = self::ml_array::optional_at(&values, self::ml_array::number_index(index));
                                {
                                    let source_ml_source1_comparison_operand45 = String::from("}");
                                    match (source_ml_source1_comparison_operand44.clone(), source_ml_source1_comparison_operand45.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            }) && opened_3) {
                                {
                                    let depth_4 = (depth_3 - 1f64);
                                    if (depth_4 == (0f64)) {
                                        self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Return(if selected_3 {
                                            1f64
                                        } else {
                                            0f64
                                        })
                                    } else {
                                        {
                                            let index_2 = (index + 1f64);
                                            {
                                                (depth, opened, selected, index) = (depth_4, opened_3, selected_3, index_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        (depth, opened, selected, index) = (depth_3, opened_3, selected_3, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Result::MlTemporaryReadRustOrdinaryMatchCompletionPresenceLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result {
    MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Done,
    MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Return(bool),
}

pub fn source_ml_temporary_test_rust_ordinary_completion_transferred_result_loop8(mut source_ml_temporary_values7: Vec<bool>, mut source_ml_temporary_index7: self::ml::Big) -> self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result {
    loop {
        return if (source_ml_temporary_index7 < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))) {
            {
                let transfer = self::ml_array::at(&source_ml_temporary_values7, source_ml_temporary_index7.to_index());
                if !transfer {
                    self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Return(false)
                } else {
                    {
                        let source_ml_temporary_index7_2 = source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index7 = source_ml_temporary_index7_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Result::MlTemporaryTestRustOrdinaryCompletionTransferredResultLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionCompletionPresenceLoop9Result {
    MlTemporaryReadRustFunctionCompletionPresenceLoop9Done,
    MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(f64),
}

pub fn source_ml_temporary_read_rust_function_completion_presence_loop9(mut values: Vec<String>, mut depth: f64, mut index: f64) -> self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| String::from(""));
                if (((value == "return") || (value == "abort")) || (value == "try")) {
                    self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(1f64)
                } else {
                    {
                        let depth_3 = if (value == "{") {
                            {
                                let depth_2 = (depth + 1f64);
                                depth_2
                            }
                        } else {
                            depth
                        };
                        if (value == "}") {
                            {
                                let depth_4 = (depth_3 - 1f64);
                                if (depth_4 == (0f64)) {
                                    self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Return(0f64)
                                } else {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            (depth, index) = (depth_4, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFunctionCompletionPresenceLoop9Result::MlTemporaryReadRustFunctionCompletionPresenceLoop9Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("value"))) == (true)), "assertion 1");
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("mixed"))) == (true)), "assertion 2");
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("returned"))) == (false)), "assertion 3");
    assert!(((self::test_rust_ordinary_completion_falls_through(String::from("aborted"))) == (false)), "assertion 4");
    assert!(((self::read_rust_ordinary_completion_snapshot_error(99999f64, 1f64, 16777215f64, 1f64)) == ""), "assertion 5");
    assert!(((self::read_rust_ordinary_completion_snapshot_error(100000f64, 1f64, 0f64, 0f64)) == "the actual completion exit snapshots exceed their finite source budget"), "assertion 6");
    assert!(((self::read_rust_ordinary_completion_snapshot_error(0f64, 0f64, 16777216f64, 1f64)) == "the actual completion exit snapshots exceed their finite source budget"), "assertion 7");
    assert!(((self::test_rust_ordinary_completion_transferred_result(Vec::<bool>::new())) == (false)), "assertion 8");
    assert!(((self::test_rust_ordinary_completion_transferred_result(vec![true])) == (true)), "assertion 9");
    assert!(((self::test_rust_ordinary_completion_transferred_result(vec![true, false])) == (false)), "assertion 10");
    assert!(((self::read_rust_ordinary_completion_construction(String::from("match"), true)) == "match"), "assertion 11");
    assert!(((self::read_rust_ordinary_completion_construction(String::from("match"), false)) == "match"), "assertion 12");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("identifier"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("return"), String::from("}")]), 1f64), "assertion 13");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("string"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("return"), String::from("}")]), 0f64), "assertion 14");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("identifier"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("r#return"), String::from("}")]), 0f64), "assertion 15");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("punct"), String::from("punct")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("?"), String::from("}")]), 1f64), "assertion 16");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier"), String::from("identifier"), String::from("punct"), String::from("identifier")], vec![String::from("match"), String::from("value"), String::from("{"), String::from("return")]), (-1f64)), "assertion 17");
    assert!(self::ml_number::same_value(self::read_rust_ordinary_match_completion_presence(vec![String::from("identifier")], vec![String::from("r#match")]), 0f64), "assertion 18");
    assert!(((self::read_rust_ordinary_match_arm_count_error(1f64)) == ""), "assertion 19");
    assert!(((self::read_rust_ordinary_match_arm_count_error(256f64)) == ""), "assertion 20");
    assert!(((self::read_rust_ordinary_match_arm_count_error(257f64)) == "the complete ordinary match arms exceed their source budget"), "assertion 21");
    assert!(((self::read_rust_ordinary_match_arm_count_error(0f64)) == "the complete ordinary match arms exceed their source budget"), "assertion 22");
    assert!(((self::read_rust_ordinary_completion_alternative_kind(String::from("broken"), String::from("value"))) == "mixed"), "assertion 23");
    assert!(((self::read_rust_ordinary_completion_alternative_kind(String::from("returned"), String::from("continued"))) == "completed loop alternatives"), "assertion 24");
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
fn all_literal_owned_record_completion_controls() { main(); }
