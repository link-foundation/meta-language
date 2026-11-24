// Generated from complete js/src/translation/reference-method-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_result(kind: &str, operation: &str) -> String {
    {
        let ml_s1 = if kind == "array" {
            if (operation == "push") || (operation == "unshift") {
                self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Return(
                    String::from("length"),
                )
            } else if (operation == "pop") || (operation == "shift") {
                self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Return(
                    String::from("optional element"),
                )
            } else if operation == "reverse" {
                self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Return(
                    String::from("receiver"),
                )
            } else {
                self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Next
            }
        } else {
            self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Next
        };
        match ml_s1 {
            self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Next => {
                if ((kind == "set") && (operation == "add"))
                    || ((kind == "map") && (operation == "set"))
                {
                    String::from("receiver")
                } else {
                    {
                        let ml_s2 = if (kind == "set") || (kind == "map") {
                            if (operation == "has") || (operation == "delete") {
                                self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Return(String::from("boolean"))
                            } else if operation == "clear" {
                                self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Return(String::from("unit"))
                            } else if (kind == "map") && (operation == "get") {
                                self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Return(String::from("optional value"))
                            } else {
                                self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Next
                            }
                        } else {
                            self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Next
                        };
                        match ml_s2 {
                            self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Next => {
                                String::new()
                            }
                            self::MlReadReferenceMethodResultJoin2::MlReadReferenceMethodResultJoin2Return(ml_result2) => {
                                ml_result2
                            }
                        }
                    }
                }
            }
            self::MlReadReferenceMethodResultJoin1::MlReadReferenceMethodResultJoin1Return(
                ml_result2_2,
            ) => ml_result2_2,
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_reference_method_operation(
    expression: &str,
    operation: &str,
    receiver: &str,
    reference: bool,
) -> bool {
    ((expression == "method") && reference)
        && (!(self::read_reference_method_result(receiver, operation)).is_empty())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_refusal(kind: &str, operation: &str) -> String {
    if ((kind == "map") || (kind == "set"))
        && (((operation == "keys") || (operation == "values")) || (operation == "entries"))
    {
        String::from(
            "live collection iteration needs ordered entries and tombstones rather than a snapshot",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_callback_effect(expression: &str, operation: &str) -> f64 {
    if ((((expression == "method")
        && (((operation == "splice") || (operation == "fill")) || (operation == "copyWithin")))
        || ((expression == "method") && operation.starts_with("writeRecordField")))
        || ((expression == "cellWrite") || (expression == "referenceWrite")))
        || ((((expression == "method") && (operation != "has")) && (operation != "get"))
            && (((!(self::read_reference_method_result("array", operation)).is_empty())
                || (!(self::read_reference_method_result("map", operation)).is_empty()))
                || (!(self::read_reference_method_result("set", operation)).is_empty())))
    {
        1f64
    } else if (expression == "call") || (expression == "functionRef") {
        2f64
    } else if expression == "apply" {
        3f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_snapshot_callback_parameters(operation: &str, count: f64) -> bool {
    count < (if operation == "reduce" { 4f64 } else { 3f64 })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_callback_operand_fields(expression: &str, ordinary: &str) -> String {
    if expression == "lambda" {
        String::from("body")
    } else if expression == "let" {
        String::from("value body")
    } else if expression == "match" {
        String::from("scrutinee cases")
    } else if expression == "outCons" {
        String::from("head tail")
    } else {
        ordinary.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_required_arguments(kind: &str, operation: &str) -> f64 {
    if (kind == "map") && (operation == "set") {
        2f64
    } else if (((operation == "add") || (operation == "get")) || (operation == "has"))
        || (operation == "delete")
    {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_argument_role(kind: &str, operation: &str, position: f64) -> String {
    if (kind == "array") && ((operation == "push") || (operation == "unshift")) {
        String::from("element")
    } else if (((kind == "set") || (kind == "map")) && (position == 0f64)) && (operation != "clear")
    {
        String::from("key")
    } else if ((kind == "map") && (operation == "set")) && (position == 1f64) {
        String::from("value")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_reference_method_key_kind(kind: &str) -> bool {
    (((((kind == "bool") || (kind == "float")) || (kind == "string")) || (kind == "int"))
        || (kind == "nat"))
        || (kind == "fixed")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_identity(kind: &str, operation: &str) -> String {
    {
        let ml_s3 = if kind == "array" {
            if operation == "push" {
                self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Return(
                    String::from("append-array-elements"),
                )
            } else if operation == "pop" {
                self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Return(
                    String::from("remove-final-array-element"),
                )
            } else if operation == "shift" {
                self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Return(
                    String::from("remove-initial-array-element"),
                )
            } else if operation == "unshift" {
                self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Return(
                    String::from("prepend-array-elements"),
                )
            } else if operation == "reverse" {
                self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Return(
                    String::from("reverse-array-elements"),
                )
            } else {
                self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Next
            }
        } else {
            self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Next
        };
        match ml_s3 {
            self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Next => {
                let ml_s4 = if kind == "set" {
                    if operation == "add" {
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Return(String::from("add-set-member"))
                    } else if operation == "delete" {
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Return(String::from("remove-set-member"))
                    } else if operation == "clear" {
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Return(String::from("clear-set-members"))
                    } else if operation == "has" {
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Return(String::from("read-set-membership"))
                    } else {
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Next
                    }
                } else {
                    self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Next
                };
                match ml_s4 {
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Next => {
                            {
                                let ml_s5 = if kind == "map" {
                                    if operation == "set" {
                                        self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Return(String::from("set-map-entry"))
                                    } else if operation == "delete" {
                                            self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Return(String::from("remove-map-entry"))
                                        } else if operation == "clear" {
                                                self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Return(String::from("clear-map-entries"))
                                            } else if operation == "get" {
                                                    self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Return(String::from("read-map-entry"))
                                                } else if operation == "has" {
                                                        self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Return(String::from("read-map-membership"))
                                                    } else {
                                                        self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Next
                                                    }
                                } else {
                                    self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Next
                                };
                                match ml_s5 {
                                    self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Next => {
                                        String::new()
                                    }
                                    self::MlReadReferenceMethodIdentityJoin5::MlReadReferenceMethodIdentityJoin5Return(ml_result5) => {
                                        ml_result5
                                    }
                                }
                            }
                        }
                        self::MlReadReferenceMethodIdentityJoin4::MlReadReferenceMethodIdentityJoin4Return(ml_result5_2) => {
                            ml_result5_2
                        }
                    }
            }
            self::MlReadReferenceMethodIdentityJoin3::MlReadReferenceMethodIdentityJoin3Return(
                ml_result5_3,
            ) => ml_result5_3,
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_statement() -> String {
    String::from(
        "Shared collection methods evaluate their actual receiver and every argument once in source order; each declared mutation changes the same allocation, preserving insertion order, SameValueZero primitive keys and the declared JavaScript result.",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_reference_method_expression(
    target: &str,
    kind: &str,
    operation: &str,
    receiver: &str,
    arguments_: &[String],
    float_key: bool,
    suffix: &str,
) -> String {
    if (self::read_reference_method_result(kind, operation)) == "" {
        String::new()
    } else if target == "JavaScript" {
        format!(
            "({}).{}({})",
            receiver.to_owned(),
            operation.to_owned(),
            arguments_.join((String::from(", ")).as_str())
        )
    } else if target == "Rust" {
        {
            let owner = format!("ml_method_owner{}", suffix.to_owned());
            {
                let values = format!("ml_method_values{}", suffix.to_owned());
                {
                    let prefix = format!("{{ let {owner} = {}; ", receiver.to_owned());
                    {
                        let index = 0f64;
                        {
                            let prefix_2 = self::ml_render_reference_method_expression_loop6(
                                arguments_, suffix, prefix, index,
                            );
                            {
                                let first = format!("ml_method_argument{}_0", suffix.to_owned());
                                {
                                    let second =
                                        format!("ml_method_argument{}_1", suffix.to_owned());
                                    if kind == "array" {
                                        if (operation == "push") || (operation == "unshift") {
                                            {
                                                let names = Vec::<String>::new();
                                                {
                                                    let index_2 = 0f64;
                                                    {
                                                        let names_2 = self::ml_render_reference_method_expression_loop7(arguments_, suffix, names, index_2);
                                                        {
                                                            let changes = if operation == "push" {
                                                                format!(
                                                                    "{values}.extend(ml_inserted);"
                                                                )
                                                            } else {
                                                                format!(
                                                                    "{values}.splice(0..0, ml_inserted);"
                                                                )
                                                            };
                                                            format!(
                                                                "{prefix_2}let ml_inserted = vec![{}]; let mut {values} = {owner}.borrow_mut(); let ml_length = {values}.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect(\"Invalid array length\"); {changes} ml_length as f64 }}",
                                                                names_2.join(
                                                                    (String::from(", ")).as_str()
                                                                )
                                                            )
                                                        }
                                                    }
                                                }
                                            }
                                        } else if operation == "pop" {
                                            format!(
                                                "{prefix_2}let ml_result = {owner}.borrow_mut().pop(); ml_result }}"
                                            )
                                        } else if operation == "shift" {
                                            format!(
                                                "{prefix_2}let mut {values} = {owner}.borrow_mut(); if {values}.is_empty() {{ None }} else {{ Some({values}.remove(0)) }} }}"
                                            )
                                        } else {
                                            format!(
                                                "{prefix_2}{owner}.borrow_mut().reverse(); {owner} }}"
                                            )
                                        }
                                    } else if operation == "clear" {
                                        format!("{prefix_2}{owner}.borrow_mut().clear(); () }}")
                                    } else {
                                        {
                                            let key = format!("ml_method_key{}", suffix.to_owned());
                                            {
                                                let prefix_3 = format!(
                                                    "{prefix_2}let {key} = {}; ",
                                                    if float_key {
                                                        format!(
                                                            "{}{}",
                                                            (format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (format!(
                                                                        "{}{}",
                                                                        (String::from("if ")),
                                                                        first
                                                                    )),
                                                                    (String::from(
                                                                        " == 0.0 { 0.0 } else { "
                                                                    ))
                                                                )),
                                                                first
                                                            )),
                                                            (String::from(" }"))
                                                        )
                                                    } else {
                                                        first
                                                    }
                                                );
                                                {
                                                    let pattern = if kind == "map" {
                                                        String::from("(ml_key, _)")
                                                    } else {
                                                        String::from("ml_key")
                                                    };
                                                    {
                                                        let equality = if float_key {
                                                            format!(
                                                                "(ml_key == &{key} || (ml_key.is_nan() && {key}.is_nan()))"
                                                            )
                                                        } else {
                                                            format!("ml_key == &{key}")
                                                        };
                                                        {
                                                            let position = format!(
                                                                "{values}.iter().position(|{pattern}| {equality})"
                                                            );
                                                            if operation == "has" {
                                                                format!(
                                                                    "{prefix_3}let {values} = {owner}.borrow(); {position}.is_some() }}"
                                                                )
                                                            } else if operation == "get" {
                                                                format!(
                                                                    "{prefix_3}let {values} = {owner}.borrow(); {position}.map(|index| {values}[index].1.clone()) }}"
                                                                )
                                                            } else if operation == "delete" {
                                                                format!(
                                                                    "{prefix_3}let mut {values} = {owner}.borrow_mut(); match {position} {{ Some(index) => {{ {values}.remove(index); true }}, None => false }} }}"
                                                                )
                                                            } else {
                                                                {
                                                                    let insertion = if kind == "map"
                                                                    {
                                                                        format!(
                                                                            "{values}.push(({key}, {second}));"
                                                                        )
                                                                    } else {
                                                                        format!(
                                                                            "{values}.push({key});"
                                                                        )
                                                                    };
                                                                    {
                                                                        let replacement = if kind
                                                                            == "map"
                                                                        {
                                                                            format!(
                                                                                "{values}[index].1 = {second};"
                                                                            )
                                                                        } else {
                                                                            String::from(
                                                                                "let _ = index;",
                                                                            )
                                                                        };
                                                                        format!(
                                                                            "{prefix_3}{{ let mut {values} = {owner}.borrow_mut(); match {position} {{ Some(index) => {{ {replacement} }}, None => {{ {insertion} }} }} }} {owner} }}"
                                                                        )
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
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadReferenceMethodResultJoin2 {
    MlReadReferenceMethodResultJoin2Next,
    MlReadReferenceMethodResultJoin2Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadReferenceMethodResultJoin1 {
    MlReadReferenceMethodResultJoin1Next,
    MlReadReferenceMethodResultJoin1Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadReferenceMethodIdentityJoin5 {
    MlReadReferenceMethodIdentityJoin5Next,
    MlReadReferenceMethodIdentityJoin5Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadReferenceMethodIdentityJoin4 {
    MlReadReferenceMethodIdentityJoin4Next,
    MlReadReferenceMethodIdentityJoin4Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadReferenceMethodIdentityJoin3 {
    MlReadReferenceMethodIdentityJoin3Next,
    MlReadReferenceMethodIdentityJoin3Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_reference_method_expression_loop6(
    arguments_: &[String],
    suffix: &str,
    mut prefix: String,
    mut index: f64,
) -> String {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let prefix_2 = format!(
                    "{prefix}let ml_method_argument{}_{} = {}; ",
                    suffix.to_owned(),
                    self::ml_number::js_number(index),
                    (self::ml_array::optional_at(arguments_, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new())
                );
                {
                    let index_2 = index + 1f64;
                    {
                        (prefix, index) = (prefix_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { prefix.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_reference_method_expression_loop7(
    arguments_: &[String],
    suffix: &str,
    mut names: Vec<String>,
    mut index: f64,
) -> Vec<String> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![format!(
                            "ml_method_argument{}_{}",
                            suffix.to_owned(),
                            self::ml_number::js_number(index)
                        )],
                        names,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}
