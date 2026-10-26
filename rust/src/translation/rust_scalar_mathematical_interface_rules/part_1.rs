// Generated from complete js/src/translation/rust-scalar-mathematical-interface-rules.js. Do not edit by hand.
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
pub fn accept_rust_scalar_source_interface_model(model: &str) -> bool {
    model == "retained scalar Rust function interface 1"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_value_type(kind: &str, bits: f64, signed: bool) -> String {
    if kind == "unit" {
        String::from("()")
    } else if kind == "bool" {
        String::from("bool")
    } else if kind == "float" {
        String::from("f64")
    } else if kind == "fixed" {
        if bits == 8f64 {
            if signed {
                String::from("i8")
            } else {
                String::from("u8")
            }
        } else if bits == 16f64 {
            if signed {
                String::from("i16")
            } else {
                String::from("u16")
            }
        } else if bits == 32f64 {
            if signed {
                String::from("i32")
            } else {
                String::from("u32")
            }
        } else if bits == 64f64 {
            if signed {
                String::from("i64")
            } else {
                String::from("u64")
            }
        } else if bits == 128f64 {
            if signed {
                String::from("i128")
            } else {
                String::from("u128")
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_type(
    value_type: &str,
    owner_kind: &str,
    owner: &str,
    paths: f64,
    children: f64,
) -> String {
    if children == 0f64 {
        if value_type == "()" {
            if (owner_kind == "unit") && (paths == 0f64) {
                String::from("()")
            } else {
                String::new()
            }
        } else if (((owner_kind != "primitive") || (paths != 1f64)) || ((value_type).is_empty()))
            || ((((((((((((value_type != "bool") && (value_type != "f64"))
                && (value_type != "i8"))
                && (value_type != "u8"))
                && (value_type != "i16"))
                && (value_type != "u16"))
                && (value_type != "i32"))
                && (value_type != "u32"))
                && (value_type != "i64"))
                && (value_type != "u64"))
                && (value_type != "i128"))
                && (value_type != "u128"))
        {
            String::new()
        } else if value_type == owner {
            value_type.to_owned()
        } else if (value_type == "u64") && (owner == "usize") {
            String::from("usize")
        } else if (value_type == "i64") && (owner == "isize") {
            String::from("isize")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_context(language: &str, forbidden: bool) -> bool {
    (language == "Rust") && !forbidden
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_declaration(kind: &str, generated: bool, owned: bool) -> bool {
    ((kind == "fn") && !generated) && owned
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_declaration_field(field: &str, ordinary_allowed: bool) -> bool {
    ordinary_allowed && (field != "sourceLoop")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_position(
    signature_model: &str,
    source_type: &str,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (((signature_model == "Rust source function signature 2") && (!(source_type).is_empty()))
        && !invalid_wrappers)
        && !borrowed_result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_expression(kind: &str) -> bool {
    ((((((((kind == "lit") || (kind == "unit")) || (kind == "var")) || (kind == "let"))
        || (kind == "if"))
        || (kind == "unary"))
        || (kind == "binary"))
        || (kind == "cast"))
        || (kind == "call")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_expression_field(kind: &str, field: &str) -> bool {
    if ((field == "k") || (field == "type")) || (field == "span") {
        self::accept_rust_scalar_source_expression(kind)
    } else if kind == "lit" {
        field == "value"
    } else if kind == "var" {
        field == "name"
    } else if kind == "let" {
        ((field == "name") || (field == "value")) || (field == "body")
    } else if kind == "if" {
        ((field == "cond") || (field == "then")) || (field == "else")
    } else if kind == "unary" {
        ((field == "op") || (field == "arg")) || (field == "semantics")
    } else if kind == "binary" {
        ((((((field == "op") || (field == "left")) || (field == "right")) || (field == "domain"))
            || (field == "semantics"))
            || (field == "rounding"))
            || (field == "byZero")
    } else if kind == "call" {
        (field == "fn") || (field == "args")
    } else if kind == "cast" {
        (((field == "arg") || (field == "from")) || (field == "to")) || (field == "flavor")
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_scalar_source_call(
    owned: bool,
    argument_types: bool,
    result_type: bool,
    static_borrow: bool,
) -> bool {
    ((owned && argument_types) && result_type) && !static_borrow
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_budget(visits: f64) -> bool {
    ((0f64..=100_000f64).contains(&visits)) && ((visits % 1f64) == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_scalar_source_conversion(
    value: &str,
    source_type: &str,
    into_source: bool,
) -> String {
    if source_type == "usize" {
        format!(
            "({}) as {}",
            value.to_owned(),
            if into_source {
                String::from("usize")
            } else {
                String::from("u64")
            }
        )
    } else if source_type == "isize" {
        format!(
            "({}) as {}",
            value.to_owned(),
            if into_source {
                String::from("isize")
            } else {
                String::from("i64")
            }
        )
    } else {
        value.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_argument_name() -> String {
    String::from("ml_source_argument")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_target_guard() -> String {
    String::from(
        "#[cfg(not(target_pointer_width = \"64\"))]\ncompile_error!(\"retained scalar Rust function interface 1 requires target_pointer_width=64\");",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_encoding() -> String {
    String::from("retained-scalar-source-interface")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_statement() -> String {
    String::from(
        "owned version-two scalar headers retain source primitive identities; calls evaluate each argument once in order before borrowing copies; usize/isize use exact 64-bit boundary conversions and other pointer widths refuse",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_bitwise_operation(symbol: &str) -> String {
    if symbol == "&" {
        String::from("conjoinRustPrimitiveBits")
    } else if symbol == "|" {
        String::from("disjoinRustPrimitiveBits")
    } else if symbol == "^" {
        String::from("excludeEqualRustPrimitiveBits")
    } else if symbol == "!" {
        String::from("complementRustPrimitiveBits")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_bitwise_symbol(operation: &str) -> String {
    if operation == "conjoinRustPrimitiveBits" {
        String::from("&")
    } else if operation == "disjoinRustPrimitiveBits" {
        String::from("|")
    } else if operation == "excludeEqualRustPrimitiveBits" {
        String::from("^")
    } else if operation == "complementRustPrimitiveBits" {
        String::from("!")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_bitwise_arity(operation: &str) -> f64 {
    {
        let symbol = self::read_rust_primitive_bitwise_symbol(operation);
        if symbol == "!" {
            1f64
        } else if (symbol).is_empty() {
            0f64
        } else {
            2f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_rust_primitive_bitwise_operation(expression: &str, operation: &str) -> bool {
    (expression == "math") && ((self::read_rust_primitive_bitwise_arity(operation)) > 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_bitwise_type(kind: &str, bits: f64) -> bool {
    (kind == "bool")
        || ((kind == "fixed")
            && (((((bits == 8f64) || (bits == 16f64)) || (bits == 32f64)) || (bits == 64f64))
                || (bits == 128f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_bitwise_precedence(symbol: &str, level: f64) -> bool {
    (((level == 0f64) && (symbol == "|")) || ((level == 1f64) && (symbol == "^")))
        || ((level == 2f64) && (symbol == "&"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_bitwise_integer_operation(operation: &str) -> String {
    if operation == "conjoinRustPrimitiveBits" {
        String::from("conjoinBigIntegerBits")
    } else if operation == "disjoinRustPrimitiveBits" {
        String::from("disjoinBigIntegerBits")
    } else if operation == "excludeEqualRustPrimitiveBits" {
        String::from("excludeEqualBigIntegerBits")
    } else if operation == "complementRustPrimitiveBits" {
        String::from("complementBigIntegerBits")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_primitive_bitwise_integer_argument(
    target: &str,
    signed: bool,
    argument: &str,
) -> String {
    if !signed && (target == "Lean") {
        format!("(Int.ofNat ({}))", argument.to_owned())
    } else if !signed && (target == "Rocq") {
        format!("(Z.of_N ({}))", argument.to_owned())
    } else {
        argument.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_rust_primitive_bitwise_expression(
    target: &str,
    operation: &str,
    kind: &str,
    bits: f64,
    signed: bool,
    arguments_: &[String],
    integer: &str,
    modulus: &str,
) -> String {
    {
        let symbol = self::read_rust_primitive_bitwise_symbol(operation);
        if !(self::accept_rust_primitive_bitwise_type(kind, bits))
            || ((arguments_.len() as f64) != (self::read_rust_primitive_bitwise_arity(operation)))
        {
            String::new()
        } else {
            {
                let left =
                    (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| String::new());
                {
                    let right = (self::ml_array::optional_at(
                        arguments_,
                        self::ml_array::number_index(1f64),
                    ))
                    .unwrap_or_else(|| String::new());
                    if target == "Rust" {
                        if symbol == "!" {
                            format!("(!({left}))")
                        } else {
                            format!("(({left}) {symbol} ({right}))")
                        }
                    } else if kind == "bool" {
                        if target == "JavaScript" {
                            if symbol == "!" {
                                format!("(!({left}))")
                            } else {
                                format!(
                                    "((ml_bit_left, ml_bit_right) => {})({left}, {right})",
                                    if symbol == "&" {
                                        String::from("ml_bit_left && ml_bit_right")
                                    } else if symbol == "|" {
                                        String::from("ml_bit_left || ml_bit_right")
                                    } else {
                                        String::from("ml_bit_left !== ml_bit_right")
                                    }
                                )
                            }
                        } else if target == "Rocq" {
                            format!(
                                "({} {left}{})",
                                if symbol == "&" {
                                    String::from("andb")
                                } else if symbol == "|" {
                                    String::from("orb")
                                } else if symbol == "^" {
                                    String::from("xorb")
                                } else {
                                    String::from("negb")
                                },
                                if symbol == "!" {
                                    String::new()
                                } else {
                                    format!("{}{}", (String::from(" ")), right)
                                }
                            )
                        } else if target == "Lean" {
                            if symbol == "!" {
                                format!("(!({left}))")
                            } else {
                                format!(
                                    "((fun (ml_bit_left ml_bit_right : Bool) => {}) ({left}) ({right}))",
                                    if symbol == "&" {
                                        String::from("ml_bit_left && ml_bit_right")
                                    } else if symbol == "|" {
                                        String::from("ml_bit_left || ml_bit_right")
                                    } else {
                                        String::from("ml_bit_left != ml_bit_right")
                                    }
                                )
                            }
                        } else {
                            String::new()
                        }
                    } else if target == "JavaScript" {
                        if symbol == "!" {
                            format!(
                                "BigInt.{}({}, ~({left}))",
                                if signed {
                                    String::from("asIntN")
                                } else {
                                    String::from("asUintN")
                                },
                                self::ml_number::js_number(bits)
                            )
                        } else {
                            format!("(({left}) {symbol} ({right}))")
                        }
                    } else if target == "Lean" {
                        if signed {
                            integer.to_owned()
                        } else {
                            format!(
                                "(Int.toNat (Int.emod {} ({} : Int)))",
                                integer.to_owned(),
                                modulus.to_owned()
                            )
                        }
                    } else if target == "Rocq" {
                        if signed {
                            integer.to_owned()
                        } else {
                            format!(
                                "(Z.to_N (Z.modulo {} ({}%Z)))",
                                integer.to_owned(),
                                modulus.to_owned()
                            )
                        }
                    } else {
                        String::new()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_mathematical_interface_model(model: &str) -> bool {
    model == "retained mathematical Rust function interface 1"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_mathematical_expression(kind: &str) -> bool {
    (kind == "math") || self::accept_rust_scalar_source_expression(kind)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_mathematical_expression_field(kind: &str, field: &str) -> bool {
    if kind == "math" {
        ((((field == "k") || (field == "type")) || (field == "span")) || (field == "op"))
            || (field == "args")
    } else {
        self::accept_rust_scalar_source_expression_field(kind, field)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_mathematical_operation(
    operation: &str,
    arguments_count: f64,
    result: &str,
) -> bool {
    {
        let arity = self::read_rust_primitive_bitwise_arity(operation);
        if arity > 0f64 {
            ((arguments_count).partial_cmp(&(arity)) == Some(std::cmp::Ordering::Equal))
                && ((result == "bool") || (result == "fixed"))
        } else if (((operation == "abs") || (operation == "floor")) || (operation == "ceil"))
            || (operation == "trunc")
        {
            (arguments_count == 1f64) && (result == "float")
        } else {
            (((operation == "isNaN") || (operation == "isFinite")) && (arguments_count == 1f64))
                && (result == "bool")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_mathematical_encoding() -> String {
    String::from("retained-mathematical-source-interface")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_mathematical_statement() -> String {
    String::from(
        "retained version-two scalar source headers and checked closed calls compose with primitive Rust bit operations and the admitted binary64 absolute, floor, ceiling, truncation and predicate operations; every operand remains a checked scalar and earlier interface models retain their scope",
    )
}
