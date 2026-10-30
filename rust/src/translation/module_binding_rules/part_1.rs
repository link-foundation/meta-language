// Generated from complete js/src/translation/module-binding-rules.js. Do not edit by hand.
/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                b
            } else {
                a
            }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                a
            } else {
                b
            }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index.and_then(|index| values.get(index)).map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
assert!(!(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))), "{}", "invalid required Number fact column position");

        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).unwrap_or_else(self::refuse_required_number_fact)
    
}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    {
assert!(!(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))), "{}", "invalid required text fact column position");

        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).unwrap_or_else(self::refuse_required_text_fact)
    
}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
assert!(!(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))), "{}", "invalid required Boolean fact column position");

        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).unwrap_or_else(self::refuse_required_boolean_fact)
    
}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_module_binding_field(starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], owner: f64, label: &str) -> f64 {
    {
        let result = -1f64;
        {
            let at = self::read_required_number_fact(starts, owner);
            {
                let source_ml_source1_s1 = self::source_ml_temporary_read_module_binding_field_loop1(starts, counts, labels, targets, owner, label, result, at);
                match source_ml_source1_s1 {
                    self::MlTemporaryReadModuleBindingFieldLoop1Result::MlTemporaryReadModuleBindingFieldLoop1Done(result_2) => {
                        result_2
                    }
                    self::MlTemporaryReadModuleBindingFieldLoop1Result::MlTemporaryReadModuleBindingFieldLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_module_binding_fields(terms: &[String], node_starts: &[f64], node_ends: &[f64], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64]) -> Vec<f64> {
    if (((((terms.len() as f64) != (node_starts.len() as f64)) || ((terms.len() as f64) != (node_ends.len() as f64))) || ((terms.len() as f64) != (starts.len() as f64))) || ((terms.len() as f64) != (counts.len() as f64))) || ((labels.len() as f64) != (targets.len() as f64)) {
        vec![-1f64, 1f64, 0f64]
    } else {
        {
            let plan = Vec::<f64>::new();
            {
                let node = 0f64;
                {
                    let source_ml_source1_s2 = self::source_ml_temporary_plan_java_script_module_binding_fields_loop2(terms, node_starts, node_ends, starts, counts, labels, targets, plan, node);
                    match source_ml_source1_s2 {
                        self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Done(plan_2) => {
                            plan_2
                        }
                        self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_module_binding_range_role(plan: &[f64], start: f64, end: f64) -> f64 {
    {
assert!(!(((plan.len() as f64) % 3f64) != 0f64), "{}", "module binding ranges need complete triples");

        {
            let selected = 0f64;
            {
                let width = f64::INFINITY;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s3 = self::source_ml_temporary_read_source_module_binding_range_role_loop6(plan, start, end, selected, width, at);
                        match source_ml_source1_s3 {
                            self::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Result::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Done(selected_2) => {
                                selected_2
                            }
                            self::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Result::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Return(source_ml_temporary_result6) => {
                                source_ml_temporary_result6
                            }
                        }
                    }
                }
            }
        }
    
}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_binding_rename_affixes(role: f64, shorthand: bool, original: &str) -> Vec<String> {
    if role == 3f64 {
        panic!("{}", "an external module name is not a local binding occurrence")
    } else if role == 1f64 {
            vec![format!("{} as ", original.to_owned()), String::new()]
        } else if role == 2f64 {
                vec![String::new(), format!(" as {}", original.to_owned())]
            } else if shorthand {
                    vec![format!("{}: ", original.to_owned()), String::new()]
                } else {
                    vec![String::new(), String::new()]
                }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_module_binding_error(code: f64) -> String {
    if code == 1f64 {
        String::from("module bindings need complete actual CST field columns")
    } else if code == 2f64 {
            String::from("a module binding field needs one actual owner")
        } else if code == 3f64 {
                String::from("a module specifier needs its actual name and optional alias fields")
            } else if code == 4f64 {
                    String::from("unknown module specifier field requires an explicit source profile")
                } else if code == 5f64 {
                        String::from("an external module name is not a local binding occurrence")
                    } else {
                        String::from("unknown module binding source error")
                    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadModuleBindingFieldLoop1Result {
    MlTemporaryReadModuleBindingFieldLoop1Done(f64),
    MlTemporaryReadModuleBindingFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn source_ml_temporary_read_module_binding_field_loop1(starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], mut owner: f64, label: &str, mut result: f64, mut at: f64) -> self::MlTemporaryReadModuleBindingFieldLoop1Result {
    loop {
        return if at < (self::read_required_number_fact(starts, owner) + self::read_required_number_fact(counts, owner)) {
            if (self::read_required_text_fact(labels, at)) == label {
                if result >= 0f64 {
                    self::MlTemporaryReadModuleBindingFieldLoop1Result::MlTemporaryReadModuleBindingFieldLoop1Return(-2f64)
                } else {
                    {
                        let result_2 = self::read_required_number_fact(targets, at);
                        {
                            let at_2 = at + 1f64;
                            {
                                (result, at) = (result_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadModuleBindingFieldLoop1Result::MlTemporaryReadModuleBindingFieldLoop1Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result {
    MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Done(Vec<f64>),
    MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4 {
    MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Next(bool),
    MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Result {
    MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Done(bool),
    MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn source_ml_temporary_plan_java_script_module_binding_fields_loop5(terms: &[String], node_starts: &[f64], node_ends: &[f64], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], mut node: f64, mut external: bool, mut owner: f64) -> self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Result {
    loop {
        return if owner < (terms.len() as f64) {
            if (((self::read_required_text_fact(terms, owner)) == "export_statement") && ((self::read_required_number_fact(node_starts, owner)) <= (self::read_required_number_fact(node_starts, node)))) && ((self::read_required_number_fact(node_ends, owner)) >= (self::read_required_number_fact(node_ends, node))) {
                {
                    let source = self::read_module_binding_field(starts, counts, labels, targets, owner, "source");
                    if source == -2f64 {
                        self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Return(vec![-1f64, 2f64, 0f64])
                    } else {
                        if source >= 0f64 {
                            {
                                let external_2 = true;
                                {
                                    let owner_2 = owner + 1f64;
                                    {
                                        (external, owner) = (external_2, owner_2);
                                        continue;
                                    }
                                }
                            }
                        }
{
                            {
                                let owner_3 = owner + 1f64;
                                {
                                    owner = owner_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let owner_4 = owner + 1f64;
                    {
                        owner = owner_4;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Done(external)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Result {
    MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Done,
    MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_module_binding_fields_loop3(starts: &[f64], counts: &[f64], labels: &[String], mut node: f64, mut at: f64) -> self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Result {
    loop {
        return if at < (self::read_required_number_fact(starts, node) + self::read_required_number_fact(counts, node)) {
            if ((self::read_required_text_fact(labels, at)) != "name") && ((self::read_required_text_fact(labels, at)) != "alias") {
                self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Return(vec![-1f64, 4f64, 0f64])
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn source_ml_temporary_plan_java_script_module_binding_fields_loop2(terms: &[String], node_starts: &[f64], node_ends: &[f64], starts: &[f64], counts: &[f64], labels: &[String], targets: &[f64], mut plan: Vec<f64>, mut node: f64) -> self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result {
    loop {
        return if node < (terms.len() as f64) {
            {
                let term = self::read_required_text_fact(terms, node);
                if term == "namespace_export" {
                    {
                        let plan_2 = {
                            let source_ml_source1_concat_value4 = plan;
                            {
                                let source_ml_source1_concat_value5 = vec![self::read_required_number_fact(node_starts, node), self::read_required_number_fact(node_ends, node), 3f64];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value4), source_ml_source1_concat_value5)
                            }
                        };
                        {
                            let node_2 = node + 1f64;
                            {
                                (plan, node) = (plan_2, node_2);
                                continue;
                            }
                        }
                    }
                }
{
                    if (term != "import_specifier") && (term != "export_specifier") {
                        {
                            let node_3 = node + 1f64;
                            {
                                node = node_3;
                                continue;
                            }
                        }
                    }
{
                        {
                            let name = self::read_module_binding_field(starts, counts, labels, targets, node, "name");
                            {
                                let alias = self::read_module_binding_field(starts, counts, labels, targets, node, "alias");
                                {
                                    let at = self::read_required_number_fact(starts, node);
                                    {
                                        let source_ml_source1_s6 = self::source_ml_temporary_plan_java_script_module_binding_fields_loop3(starts, counts, labels, node, at);
                                        match source_ml_source1_s6 {
                                            self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Done => {
                                                if (name == -2f64) || (alias == -2f64) {
                                                    self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Return(vec![-1f64, 2f64, 0f64])
                                                } else if ((((!(self::ml_math::is_safe_integer(name)) || (name < 0f64)) || (name >= (terms.len() as f64))) || !(self::ml_math::is_safe_integer(alias))) || (alias < -1f64)) || (alias >= (terms.len() as f64)) {
                                                        self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Return(vec![-1f64, 3f64, 0f64])
                                                    } else {
                                                        {
                                                            let external = (term == "import_specifier") && (alias >= 0f64);
                                                            {
                                                                let source_ml_source1_s8 = if term == "export_specifier" {
                                                                    {
                                                                        let owner = 0f64;
                                                                        {
                                                                            let source_ml_source1_s9 = self::source_ml_temporary_plan_java_script_module_binding_fields_loop5(terms, node_starts, node_ends, starts, counts, labels, targets, node, external, owner);
                                                                            match source_ml_source1_s9 {
                                                                                self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Done(external_2) => {
                                                                                    self::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Next(external_2)
                                                                                }
                                                                                self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop5Return(source_ml_temporary_result5) => {
                                                                                    self::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Return(source_ml_temporary_result5)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Next(external)
                                                                };
                                                                match source_ml_source1_s8 {
                                                                    self::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Next(external_3) => {
                                                                        {
                                                                            let plan_3 = {
                                                                                let source_ml_source1_concat_value10 = plan;
                                                                                {
                                                                                    let source_ml_source1_concat_value11 = vec![self::read_required_number_fact(node_starts, name), self::read_required_number_fact(node_ends, name), if external_3 {
                                                                                        3f64
                                                                                    } else if alias < 0f64 {
                                                                                            if term == "import_specifier" {
                                                                                                1f64
                                                                                            } else {
                                                                                                2f64
                                                                                            }
                                                                                        } else {
                                                                                            0f64
                                                                                        }];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value10), source_ml_source1_concat_value11)
                                                                                }
                                                                            };
                                                                            if (alias >= 0f64) && (term == "export_specifier") {
                                                                                {
                                                                                    let plan_4 = {
                                                                                        let source_ml_source1_concat_value12 = plan_3;
                                                                                        {
                                                                                            let source_ml_source1_concat_value13 = vec![self::read_required_number_fact(node_starts, alias), self::read_required_number_fact(node_ends, alias), 3f64];
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value12), source_ml_source1_concat_value13)
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let node_4 = node + 1f64;
                                                                                        {
                                                                                            (plan, node) = (plan_4, node_4);
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
{
                                                                                {
                                                                                    let node_5 = node + 1f64;
                                                                                    {
                                                                                        (plan, node) = (plan_3, node_5);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4::MlTemporaryPlanJavaScriptModuleBindingFieldsJoin4Return(source_ml_temporary_result4) => {
                                                                        self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Return(source_ml_temporary_result4)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                            }
                                            self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop3Return(source_ml_temporary_result5_2) => {
                                                self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Return(source_ml_temporary_result5_2)
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
            self::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Result::MlTemporaryPlanJavaScriptModuleBindingFieldsLoop2Done(plan)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSourceModuleBindingRangeRoleLoop6Result {
    MlTemporaryReadSourceModuleBindingRangeRoleLoop6Done(f64),
    MlTemporaryReadSourceModuleBindingRangeRoleLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_module_binding_range_role_loop6(plan: &[f64], mut start: f64, mut end: f64, mut selected: f64, mut width: f64, mut at: f64) -> self::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Result {
    loop {
        return if at < (plan.len() as f64) {
            {
                let begin = self::read_required_number_fact(plan, at);
                {
                    let finish = self::read_required_number_fact(plan, (at + 1f64));
                    {
                        let role = self::read_required_number_fact(plan, (at + 2f64));
                        if (begin <= start) && (finish >= end) {
                            if role == 3f64 {
                                self::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Result::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Return(3f64)
                            } else {
                                if (finish - begin) < width {
                                    {
                                        let selected_2 = role;
                                        {
                                            let width_2 = finish - begin;
                                            {
                                                let at_2 = at + 3f64;
                                                {
                                                    (selected, width, at) = (selected_2, width_2, at_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
{
                                    {
                                        let at_3 = at + 3f64;
                                        {
                                            at = at_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let at_4 = at + 3f64;
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
            self::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Result::MlTemporaryReadSourceModuleBindingRangeRoleLoop6Done(selected)
        };
    }
}
