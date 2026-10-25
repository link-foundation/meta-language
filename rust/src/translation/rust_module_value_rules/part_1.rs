// Generated from complete js/src/translation/rust-module-value-rules.js. Do not edit by hand.
/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript `Number::toString`: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
    #[must_use]
    pub fn js_number(x: f64) -> String {
        if x.is_nan() {
            return String::from("NaN");
        }
        if x.is_infinite() {
            return String::from(if x > 0.0 { "Infinity" } else { "-Infinity" });
        }
        if x == 0.0 {
            return String::from("0");
        }
        let (mut digits, n) = decimal(&format!("{:e}", x.abs()));
        // Rust's shortest digits round a tie away from zero; ECMAScript takes the even neighbour.
        let (exact, _) = decimal(&format!("{:.800e}", x.abs()));
        let rest = &exact[digits.len().min(exact.len())..];
        if digits.len() > 1 && rest.starts_with('5') && rest[1..].bytes().all(|b| b == b'0') {
            let last = digits.as_bytes()[digits.len() - 1] - b'0';
            let down = exact[..digits.len()].to_string();
            let other = if digits == down {
                increment(&down)
            } else {
                Some(down)
            };
            if let Some(other) = other {
                let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
                if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                    digits = other.trim_end_matches('0').to_string();
                }
            }
        }
        let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");
        let body = if k <= n && n <= 21 {
            format!("{digits}{}", "0".repeat((n - k) as usize))
        } else if 0 < n && n <= 21 {
            format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
        } else if -6 < n && n <= 0 {
            format!("0.{}{digits}", "0".repeat((-n) as usize))
        } else {
            let sign = if n - 1 < 0 { '-' } else { '+' };
            let power = (n - 1).abs();
            if k == 1 {
                format!("{digits}e{sign}{power}")
            } else {
                format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..])
            }
        };
        if x < 0.0 { format!("-{body}") } else { body }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    #[must_use]
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// `SameValue`, as `Object.is` and `assert.strictEqual` compare: NaN equals
    /// NaN, and 0 differs from -0.
    #[must_use]
    pub const fn same_value(a: f64, b: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            a.is_nan() && b.is_nan()
        } else {
            a.to_bits() == b.to_bits()
        }
    }

    /// A decimal digit string plus one, or None when it gains a digit.
    fn increment(digits: &str) -> Option<String> {
        let mut bytes = digits.as_bytes().to_vec();
        for index in (0..bytes.len()).rev() {
            if bytes[index] == b'9' {
                bytes[index] = b'0';
            } else {
                bytes[index] += 1;
                return String::from_utf8(bytes).ok();
            }
        }
        None
    }

    /// The significant digits without trailing zeros, and the exponent n with
    /// value 0.digits × 10^n, of Rust's {:e} text.
    fn decimal(text: &str) -> (String, i32) {
        let (mantissa, exponent) = text.split_once('e').expect("exponent");
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let digits = digits.trim_end_matches('0').to_string();
        let digits = if digits.is_empty() {
            String::from("0")
        } else {
            digits
        };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
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
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
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
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
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
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
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
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_value_module_item(term: &str, comment: bool, role: f64) -> bool {
    (role != 3f64)
        && ((((comment || (term == "function_item")) || (term == "const_item"))
            || (term == "attribute_item"))
            || (term == "inner_attribute_item"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_value_module_owner_kind(term: &str) -> String {
    if term == "const_item" {
        String::from("const")
    } else if term == "function_item" {
        String::from("fn")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_public_constant_name_position(kinds: &[String], values: &[String]) -> f64 {
    {
        let ml_library_inequality_1 = (kinds.len() as f64) != (values.len() as f64);
        if ml_library_inequality_1 {
            -1f64
        } else {
            {
                let position = 0f64;
                {
                    let source_ml_source2_s1 = self::source_ml_source1_temporary_read_rust_public_constant_name_position_loop1(kinds, values, position);
                    match source_ml_source2_s1 {
                    self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Done(position_2) => {
                        if (((position_2 + 3f64) >= (values.len() as f64)) || ((self::read_required_text_fact(kinds, position_2)) != "identifier")) || ((self::read_required_text_fact(values, position_2)) != "pub") {
                            -1f64
                        } else {
                            {
                                let position_3 = position_2 + 1f64;
                                if (((self::read_required_text_fact(kinds, position_3)) != "identifier") || ((self::read_required_text_fact(values, position_3)) != "const")) || ((self::read_required_text_fact(kinds, (position_3 + 1f64))) != "identifier") {
                                    -1f64
                                } else {
                                    position_3 + 1f64
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Return(source_ml_source1_temporary_result1) => {
                        source_ml_source1_temporary_result1
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_module_value_export_binding(position: f64, reserved: &[String]) -> String {
    {
        let name = format!(
            "ml_source_constant_export_{}",
            self::ml_number::js_number(position)
        );
        {
            self::source_ml_source1_temporary_read_rust_module_value_export_binding_loop3(
                reserved, name,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_module_value_export(source: &str, getter: &str, binding: &str) -> String {
    format!(
        "const {} = {}();\nexport {{ {} as {} }};",
        binding.to_owned(),
        getter.to_owned(),
        binding.to_owned(),
        source.to_owned()
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_callable_constant_application_capture(
    head: &str,
    local: bool,
    declaration: &str,
    value: &str,
) -> bool {
    (((head == "name") && !local) && (declaration == "const")) && (value == "function")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_module_value_export_bindings(
    kinds: &[String],
    reserved: &[String],
) -> Vec<String> {
    {
        let occupied = self::ml_array::set(reserved.to_vec(), |a, b| a == b);
        {
            let bindings = Vec::<String>::new();
            {
                let position = 0f64;
                {
                    self::source_ml_source1_temporary_plan_rust_module_value_export_bindings_loop4(
                        kinds,
                        &(occupied),
                        bindings,
                        position,
                    )
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result {
    MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Done(f64),
    MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Result {
    MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_public_constant_name_position_loop2(
    kinds: &[String],
    values: &[String],
    mut position: f64,
    mut depth: f64,
) -> self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Result {
    loop {
        if (position < (values.len() as f64)) && (depth > 0f64) {
            {
                let depth_4 = if (self::read_required_text_fact(kinds, position)) == "punct" {
                    if (self::read_required_text_fact(values, position)) == "[" {
                        { depth + 1f64 }
                    } else if (self::read_required_text_fact(values, position)) == "]" {
                        { depth - 1f64 }
                    } else {
                        depth
                    }
                } else {
                    depth
                };
                {
                    let position_2 = position + 1f64;
                    {
                        (position, depth) = (position_2, depth_4);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Done(position, depth)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_public_constant_name_position_loop1(
    kinds: &[String],
    values: &[String],
    mut position: f64,
) -> self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result {
    loop {
        return if ((position < (values.len() as f64))
            && ((self::read_required_text_fact(kinds, position)) == "punct"))
            && ((self::read_required_text_fact(values, position)) == "#")
        {
            {
                let position_2 = position + 1f64;
                {
                    let position_4 = if ((position_2 < (values.len() as f64))
                        && ((self::read_required_text_fact(kinds, position_2)) == "punct"))
                        && ((self::read_required_text_fact(values, position_2)) == "!")
                    {
                        { position_2 + 1f64 }
                    } else {
                        position_2
                    };
                    if ((position_4 >= (values.len() as f64))
                        || ((self::read_required_text_fact(kinds, position_4)) != "punct"))
                        || ((self::read_required_text_fact(values, position_4)) != "[")
                    {
                        self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Return(-1f64)
                    } else {
                        {
                            let depth = 1f64;
                            {
                                let position_5 = position_4 + 1f64;
                                {
                                    let source_ml_source2_s2 = self::source_ml_source1_temporary_read_rust_public_constant_name_position_loop2(kinds, values, position_5, depth);
                                    match source_ml_source2_s2 {
                                        self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop2Done(position_6, depth_2) => {
                                            {
let ml_library_inequality_1 = depth_2 != 0f64 ;
if ml_library_inequality_1 {
                                                self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Return(-1f64)
                                            } else {
                                                {
                                                    position = position_6;
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
        } else {
            self::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Result::MlSource1TemporaryReadRustPublicConstantNamePositionLoop1Done(position)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_module_value_export_binding_loop3(
    reserved: &[String],
    mut name: String,
) -> String {
    loop {
        if self::ml_array::includes(reserved, &name.clone(), 0f64, false, |a, b| a == b) {
            {
                let name_2 = format!("{name}_");
                {
                    name = name_2;
                    continue;
                }
            }
        }
        return { name.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_module_value_export_bindings_loop5(
    occupied: &[String],
    mut name: String,
) -> String {
    loop {
        return if {
            let ml_collection = occupied.to_vec();
            let ml_search = name.clone();
            ml_collection.iter().any(|ml_key| ml_key == &ml_search)
        } {
            {
                let name_2 = format!("{name}_");
                {
                    name = name_2;
                    continue;
                }
            }
        } else {
            name.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_module_value_export_bindings_loop4(
    kinds: &[String],
    occupied: &[String],
    mut bindings: Vec<String>,
    mut position: f64,
) -> Vec<String> {
    loop {
        if position < (kinds.len() as f64) {
            if (self::read_required_text_fact(kinds, position)) != "const" {
                {
                    let bindings_2 = {
                        let (ml_append_right, ml_append_left) = (vec![String::new()], bindings);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let position_2 = position + 1f64;
                        {
                            (bindings, position) = (bindings_2, position_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let name = format!(
                        "ml_source_constant_export_{}",
                        self::ml_number::js_number(position)
                    );
                    {
                        let name_2 = self::source_ml_source1_temporary_plan_rust_module_value_export_bindings_loop5(occupied, name);
                        {
                            let bindings_3 = {
                                let (ml_append_right, ml_append_left) = (vec![name_2], bindings);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let position_3 = position + 1f64;
                                {
                                    (bindings, position) = (bindings_3, position_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { bindings };
    }
}
