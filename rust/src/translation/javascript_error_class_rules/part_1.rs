// Generated from complete js/src/translation/javascript-error-class-rules.js. Do not edit by hand.
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
pub fn find_java_script_class_group_end(values: &[String], open: f64) -> f64 {
    {
        let first = (self::ml_array::optional_at(values, self::ml_array::number_index(open))).clone().unwrap_or_else(|| String::from(""));
        {
            let last = if (first == "(") {
                String::from(")")
            } else {
                if (first == "[") {
                    String::from("]")
                } else {
                    if (first == "{") {
                        String::from("}")
                    } else {
                        String::from("")
                    }
                }
            };
            if (last == "") {
                (-1f64)
            } else {
                {
                    let expected = vec![last.clone()];
                    {
                        let at = (open + 1f64);
                        {
                            let source_ml_source2_s1 = self::source_ml_source1_temporary_find_java_script_class_group_end_loop1(values, open, expected, at);
                            match source_ml_source2_s1.clone() {
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done => {
                                    (-1f64)
                                }
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(source_ml_source1_temporary_result1) => {
                                    source_ml_source1_temporary_result1
                                }
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
pub fn read_java_script_source_exception_operation(name: &str) -> String {
    if (name == "Error") {
        String::from("raiseJavaScriptError")
    } else {
        if (name == "TypeError") {
            String::from("raiseJavaScriptTypeError")
        } else {
            if (name == "RangeError") {
                String::from("raiseJavaScriptRangeError")
            } else {
                if (name == "SyntaxError") {
                    String::from("raiseJavaScriptSyntaxError")
                } else {
                    if (name == "ReferenceError") {
                        String::from("raiseJavaScriptReferenceError")
                    } else {
                        if (name == "URIError") {
                            String::from("raiseJavaScriptURIError")
                        } else {
                            if (name == "EvalError") {
                                String::from("raiseJavaScriptEvalError")
                            } else {
                                String::from("")
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
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn text(values: &[String], at: f64) -> String {
    if ((at >= (0f64)) && (at < ((values.len() as f64)))) {
        self::read_required_text_fact(values, at)
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_class_profile_error(profile: &str) -> String {
    if (profile == "actual intrinsic Error subclasses 1") {
        String::from("")
    } else {
        String::from("Error subclasses require their explicit intrinsic/realm construction profile")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_class_directory(values: &[String], kinds: &[String], first: f64) -> Vec<f64> {
    if (((!(self::ml_math::is_safe_integer(first)) || (first < (0f64))) || (((values.len() as f64)) != ((kinds.len() as f64)))) || (((values.len() as f64)) > (8192f64))) {
        vec![(-1f64)]
    } else {
        if (((((((self::text(values, first)) != "class") || ((self::text(kinds, (first + 1f64))) != "identifier")) || ((self::text(values, (first + 2f64))) != "extends")) || ((self::text(kinds, (first + 3f64))) != "identifier")) || ((self::read_java_script_source_exception_operation(&(self::text(values, (first + 3f64))))) == "")) || ((self::text(values, (first + 4f64))) != "{")) {
            vec![(-1f64)]
        } else {
            {
                let end = self::find_java_script_class_group_end(values, (first + 4f64));
                if (end < (0f64)) {
                    vec![(-1f64)]
                } else {
                    {
                        let at = (first + 5f64);
                        if (((self::text(values, at)) != "constructor") || ((self::text(values, (at + 1f64))) != "(")) {
                            vec![(-2f64)]
                        } else {
                            {
                                let open = (at + 1f64);
                                {
                                    let close = self::find_java_script_class_group_end(values, open);
                                    if ((close < (0f64)) || ((self::text(values, (close + 1f64))) != "{")) {
                                        vec![(-2f64)]
                                    } else {
                                        {
                                            let body = (close + 1f64);
                                            {
                                                let body_end = self::find_java_script_class_group_end(values, body);
                                                if ((body_end < (0f64)) || (((body_end + 1f64)) != end)) {
                                                    vec![(-2f64)]
                                                } else {
                                                    {
                                                        let at_2 = (body + 1f64);
                                                        if (((self::text(values, at_2)) != "super") || ((self::text(values, (at_2 + 1f64))) != "(")) {
                                                            vec![(-3f64)]
                                                        } else {
                                                            {
                                                                let super_end = self::find_java_script_class_group_end(values, (at_2 + 1f64));
                                                                if ((super_end < (0f64)) || ((self::text(values, (super_end + 1f64))) != ";")) {
                                                                    vec![(-3f64)]
                                                                } else {
                                                                    {
                                                                        let plan = vec![end, open, close, (at_2 + 2f64), super_end];
                                                                        {
                                                                            let at_3 = (super_end + 2f64);
                                                                            {
                                                                                let source_ml_source2_s144 = self::source_ml_source1_temporary_read_java_script_error_class_directory_loop70(values, kinds, at_3, body_end, plan);
                                                                                match source_ml_source2_s144.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Done(plan_2) => {
                                                                                        plan_2
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Return(source_ml_source1_temporary_result70) => {
                                                                                        source_ml_source1_temporary_result70
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
pub fn read_java_script_error_class_refusal(code: f64) -> String {
    if (code == ((-1f64))) {
        String::from("the Error successor needs an actual named class extending a supplied standard Error constructor")
    } else {
        if (code == ((-2f64))) {
            String::from("this Error successor needs its complete single ordinary constructor rather than unmodeled fields, methods or descriptor effects")
        } else {
            if (code == ((-3f64))) {
                String::from("a dominant explicit super call must initialize this before the ordered field body")
            } else {
                String::from("Error constructor fields require complete ordered this assignments; returns, repeated super, prototype mutation and other statements retain their source obligations")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result {
    MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done,
    MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_class_group_end_loop1(values: &[String], mut open: f64, mut expected: Vec<String>, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result {
    loop {
        return if ((at < ((values.len() as f64))) && (((at - open)) <= (8192f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| String::from(""));
                {
                    let closing = if (value == "(") {
                        String::from(")")
                    } else {
                        if (value == "[") {
                            String::from("]")
                        } else {
                            if (value == "{") {
                                String::from("}")
                            } else {
                                String::from("")
                            }
                        }
                    };
                    if (closing != "") {
                        if (((expected.len() as f64)) >= (128f64)) {
                            self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return((-1f64))
                        } else {
                            {
                                let expected_2 = {
                                    let source_ml_source2_concat_value146 = vec![closing.clone()];
                                    self::ml_array::append(self::ml_array::append(Vec::<String>::new(), expected), source_ml_source2_concat_value146)
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (expected, at) = (expected_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if (((value == ")") || (value == "]")) || (value == "}")) {
                            if (value != ((self::ml_array::optional_at(&expected, self::ml_array::number_index(((expected.len() as f64) - 1f64)))).clone().unwrap_or_else(|| String::from("")))) {
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return((-1f64))
                            } else {
                                {
                                    let expected_3 = self::ml_array::slice(&expected, 0f64, (-1f64));
                                    if (((expected_3.len() as f64)) == (0f64)) {
                                        self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(at)
                                    } else {
                                        {
                                            let at_3 = (at + 1f64);
                                            {
                                                (expected, at) = (expected_3, at_3);
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
            self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result {
    MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result {
    MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Done(f64),
    MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72 {
    MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Next(f64),
    MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_class_directory_loop71(values: &[String], mut body_end: f64, mut last: f64) -> self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result {
    loop {
        return if (last < body_end) {
            {
                let value = self::text(values, last);
                if (value == ";") {
                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Done(last)
                } else {
                    {
                        let source_ml_source2_s335 = if self::ml_array::includes(&vec![String::from("("), String::from("["), String::from("{")], &value.clone(), 0f64, false, |a, b| a == b) {
                            {
                                let group = self::find_java_script_class_group_end(values, last);
                                if ((group < (0f64)) || (group >= body_end)) {
                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Return(vec![(-4f64)])
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Next(group)
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Next(last)
                        };
                        match source_ml_source2_s335.clone() {
                            self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Next(last_2) => {
                                if ((value == "return") || (value == "super")) {
                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Return(vec![(-4f64)])
                                } else {
                                    {
                                        let last_3 = (last_2 + 1f64);
                                        {
                                            last = last_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72::MlSource1TemporaryReadJavaScriptErrorClassDirectoryJoin72Return(source_ml_source1_temporary_result72) => {
                                self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Return(source_ml_source1_temporary_result72)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Done(last)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_class_directory_loop70(values: &[String], kinds: &[String], mut at: f64, mut body_end: f64, mut plan: Vec<f64>) -> self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result {
    loop {
        return if (at < body_end) {
            if (((((self::text(values, at)) != "this") || ((self::text(values, (at + 1f64))) != ".")) || ((self::text(kinds, (at + 2f64))) != "identifier")) || ((self::text(values, (at + 3f64))) != "=")) {
                self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Return(vec![(-4f64)])
            } else {
                {
                    let name = (at + 2f64);
                    {
                        let start = (at + 4f64);
                        {
                            let last = start;
                            {
                                let source_ml_source2_s336 = self::source_ml_source1_temporary_read_java_script_error_class_directory_loop71(values, body_end, last);
                                match source_ml_source2_s336.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Done(last_2) => {
                                        if (((last_2 <= start) || (last_2 >= body_end)) || ((self::text(values, name)) == "__proto__")) {
                                            self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Return(vec![(-4f64)])
                                        } else {
                                            {
                                                let plan_2 = {
                                                    let source_ml_source2_concat_value338 = vec![name, start, last_2];
                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), plan), source_ml_source2_concat_value338)
                                                };
                                                {
                                                    let at_2 = (last_2 + 1f64);
                                                    {
                                                        (at, plan) = (at_2, plan_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop71Return(source_ml_source1_temporary_result71) => {
                                        self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Return(source_ml_source1_temporary_result71)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Result::MlSource1TemporaryReadJavaScriptErrorClassDirectoryLoop70Done(plan)
        };
    }
}
