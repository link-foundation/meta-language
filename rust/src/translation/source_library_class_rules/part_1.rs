// Generated from complete js/src/translation/source-library-class-rules.js. Do not edit by hand.
/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
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

    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9007199254740991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
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
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self { class_name: class_name.to_owned(), message_units: message.encode_utf16().collect(), message_present: true }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(kind: &str, name: &str, duplicate: bool, reserved: &[String]) -> bool {
    if (((kind != "identifier") || (name == "")) || duplicate) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s60 = self::source_ml_source1_temporary_dependency_function1_loop3(name, reserved, index);
                match source_ml_source2_s60.clone() {
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done => {
                        true
                    }
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_declaration_binding_kind(keyword: &str, kind: &str, name: &str, callable: bool) -> String {
    if !(self::dependency_function1(kind, name, false, &(Vec::<String>::new()))) {
        String::from("")
    } else {
        if (keyword == "function") {
            String::from("function")
        } else {
            if ((keyword == "const") && callable) {
                String::from("const")
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_reserved_identifier(name: &str) -> bool {
    self::ml_array::includes(&vec![String::from("await"), String::from("break"), String::from("case"), String::from("catch"), String::from("class"), String::from("const"), String::from("continue"), String::from("debugger"), String::from("default"), String::from("delete"), String::from("do"), String::from("else"), String::from("enum"), String::from("export"), String::from("extends"), String::from("false"), String::from("finally"), String::from("for"), String::from("function"), String::from("if"), String::from("import"), String::from("in"), String::from("instanceof"), String::from("new"), String::from("null"), String::from("return"), String::from("super"), String::from("switch"), String::from("this"), String::from("throw"), String::from("true"), String::from("try"), String::from("typeof"), String::from("var"), String::from("void"), String::from("while"), String::from("with"), String::from("yield"), String::from("let"), String::from("static"), String::from("implements"), String::from("interface"), String::from("package"), String::from("private"), String::from("protected"), String::from("public")], &name.to_owned(), 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function2(values: &[f64], index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function3(values: &[String], index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_class_profile_error(profile: &str, dependencies: bool, aliases: bool, shared: bool) -> String {
    if (profile == "") {
        String::from("")
    } else {
        if ((((profile == "closed local class declarations 1") && dependencies) && aliases) && shared) {
            String::from("")
        } else {
            String::from("closed class composition needs its exact supplied dependency, resolved alias and shared reference profiles")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_re_export_profile_error(profile: &str, dependencies: bool, aliases: bool) -> String {
    if ((profile == "") || (((profile == "resolved named module providers 1") && dependencies) && aliases)) {
        String::from("")
    } else {
        String::from("named re-export composition needs actual supplied dependencies and resolved aliases")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_library_re_export_name(kind: &str, name: &str) -> bool {
    (((self::dependency_function1(kind, name, false, &(Vec::<String>::new())) && !(self::accept_java_script_reserved_identifier(name))) && (name != "eval")) && (name != "arguments"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_library_entry_export_owner(exported: &str, compiled: &str) -> bool {
    ((exported != "") && (exported == compiled))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_class_binding_kind(keyword: &str, kind: &str, name: &str, callable: bool) -> String {
    if (((((keyword == "class") && self::dependency_function1(kind, name, false, &(Vec::<String>::new()))) && !(self::accept_java_script_reserved_identifier(name))) && (name != "eval")) && (name != "arguments")) {
        String::from("class")
    } else {
        self::read_source_library_declaration_binding_kind(keyword, kind, name, callable)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_library_class_export(keyword: &str, kind: &str, name: &str, callable: bool) -> bool {
    ((self::read_source_library_class_binding_kind(keyword, kind, name, callable)) != "")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_library_class_declaration_term(term: &str, kind: &str) -> bool {
    if (kind == "class") {
        (term == "class_declaration")
    } else {
        if (kind == "function") {
            ((term == "function_declaration") || (term == "generator_function_declaration"))
        } else {
            ((kind == "const") && (term == "lexical_declaration"))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_class_declaration_owner(names: &[String], starts: &[f64], ends: &[f64], witnessed: &[String], first: f64, last: f64) -> String {
    {
        let owner = String::from("");
        if ((((((names.len() as f64)) != ((starts.len() as f64))) || (((names.len() as f64)) != ((ends.len() as f64)))) || (first < (0f64))) || (last < first)) {
            String::from("")
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s74 = self::source_ml_source1_temporary_read_source_library_class_declaration_owner_loop8(names, starts, ends, witnessed, first, last, owner.clone(), at);
                    match source_ml_source2_s74.clone() {
                        self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(owner_2) => {
                            owner_2.clone()
                        }
                        self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(source_ml_source1_temporary_result8) => {
                            source_ml_source1_temporary_result8.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_library_class_helper_call(imported_owners: &[String], constructor: &str, helpers: &[String], helper: &str, receiver_owner: &str) -> bool {
    ((((constructor != "") && (constructor == receiver_owner)) && self::ml_array::includes(imported_owners, &constructor.to_owned(), 0f64, false, |a, b| a == b)) && self::ml_array::includes(helpers, &helper.to_owned(), 0f64, false, |a, b| a == b))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_library_class_static_call(imported_owners: &[String], constructor: &str, helpers: &[String], helper: &str) -> bool {
    (((constructor != "") && self::ml_array::includes(imported_owners, &constructor.to_owned(), 0f64, false, |a, b| a == b)) && self::ml_array::includes(helpers, &helper.to_owned(), 0f64, false, |a, b| a == b))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_source_library_class_edit_interval(starts: &[f64], ends: &[f64], lengths: &[f64], first: f64, last: f64) -> Vec<f64> {
    if (((((((((starts.len() as f64)) != ((ends.len() as f64))) || (((starts.len() as f64)) != ((lengths.len() as f64)))) || (((starts.len() as f64)) > (32768f64))) || !(self::ml_math::is_safe_integer(first))) || !(self::ml_math::is_safe_integer(last))) || (first < (0f64))) || (last < first)) {
        Vec::<f64>::new()
    } else {
        {
            let delta = 0f64;
            {
                let previous = 0f64;
                {
                    let low = (-1f64);
                    {
                        let high = (-1f64);
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s75 = self::source_ml_source1_temporary_project_source_library_class_edit_interval_loop9(starts, ends, lengths, first, last, delta, previous, low, high, at);
                                match source_ml_source2_s75.clone() {
                                    self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(delta_2, low_2, high_2) => {
                                        {
                                            let low_4 = if (low_2 < (0f64)) {
                                                {
                                                    let low_3 = (first - delta_2);
                                                    low_3
                                                }
                                            } else {
                                                low_2
                                            };
                                            {
                                                let high_4 = if (high_2 < (0f64)) {
                                                    {
                                                        let high_3 = (last - delta_2);
                                                        high_3
                                                    }
                                                } else {
                                                    high_2
                                                };
                                                if ((low_4 < (0f64)) || (high_4 < low_4)) {
                                                    Vec::<f64>::new()
                                                } else {
                                                    vec![low_4, high_4]
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(source_ml_source1_temporary_result9) => {
                                        source_ml_source1_temporary_result9
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDependencyFunction1Loop3Result {
    MlSource1TemporaryDependencyFunction1Loop3Done,
    MlSource1TemporaryDependencyFunction1Loop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function1_loop3(name: &str, reserved: &[String], mut index: f64) -> self::MlSource1TemporaryDependencyFunction1Loop3Result {
    loop {
        return if (index < ((reserved.len() as f64))) {
            {
                let word = self::ml_array::optional_at(reserved, self::ml_array::number_index(index));
                if {
                    let source_ml_source2_comparison_operand85 = word.clone();
                    {
                        let source_ml_source2_comparison_operand86 = None::<()>;
                        match (source_ml_source2_comparison_operand85.clone(), source_ml_source2_comparison_operand86.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    }
                } {
                    panic!("{}", "source binding reserved names need a complete text column")
                } else {
                    if ((word.clone().expect("a proven present value")) == name) {
                        self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(false)
                    } else {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result {
    MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(String),
    MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_source_library_class_declaration_owner_loop8(names: &[String], starts: &[f64], ends: &[f64], witnessed: &[String], mut first: f64, mut last: f64, mut owner: String, mut at: f64) -> self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result {
    loop {
        return if (at < ((names.len() as f64))) {
            if (((first >= (self::dependency_function2(starts, at))) && (last <= (self::dependency_function2(ends, at)))) && self::ml_array::includes(witnessed, &self::dependency_function3(names, at), 0f64, false, |a, b| a == b)) {
                if (owner != "") {
                    self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(String::from("#ambiguous"))
                } else {
                    {
                        let owner_2 = self::dependency_function3(names, at);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (owner, at) = (owner_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result {
    MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(f64, f64, f64),
    MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_source_library_class_edit_interval_loop9(starts: &[f64], ends: &[f64], lengths: &[f64], mut first: f64, mut last: f64, mut delta: f64, mut previous: f64, mut low: f64, mut high: f64, mut at: f64) -> self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result {
    loop {
        return if (at < ((starts.len() as f64))) {
            {
                let start = self::dependency_function2(starts, at);
                {
                    let end = self::dependency_function2(ends, at);
                    {
                        let length = self::dependency_function2(lengths, at);
                        if (((((!(self::ml_math::is_safe_integer(start)) || !(self::ml_math::is_safe_integer(end))) || !(self::ml_math::is_safe_integer(length))) || (start < previous)) || (end < start)) || (length < (0f64))) {
                            self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(Vec::<f64>::new())
                        } else {
                            {
                                let changed_start = (start + delta);
                                {
                                    let changed_end = (changed_start + length);
                                    {
                                        let low_3 = if ((low < (0f64)) && (first < changed_start)) {
                                            {
                                                let low_2 = (first - delta);
                                                low_2
                                            }
                                        } else {
                                            low
                                        };
                                        {
                                            let high_3 = if ((high < (0f64)) && (last < changed_start)) {
                                                {
                                                    let high_2 = (last - delta);
                                                    high_2
                                                }
                                            } else {
                                                high
                                            };
                                            {
                                                let low_5 = if (((first >= changed_start) && (first <= changed_end)) && (low_3 < (0f64))) {
                                                    {
                                                        let low_4 = start;
                                                        low_4
                                                    }
                                                } else {
                                                    low_3
                                                };
                                                {
                                                    let high_5 = if (((last >= changed_start) && (last <= changed_end)) && (high_3 < (0f64))) {
                                                        {
                                                            let high_4 = end;
                                                            high_4
                                                        }
                                                    } else {
                                                        high_3
                                                    };
                                                    {
                                                        let delta_2 = (delta + (length - (end - start)));
                                                        {
                                                            let previous_2 = end;
                                                            {
                                                                let at_2 = (at + 1f64);
                                                                {
                                                                    (delta, previous, low, high, at) = (delta_2, previous_2, low_5, high_5, at_2);
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
        } else {
            self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(delta, low, high)
        };
    }
}
