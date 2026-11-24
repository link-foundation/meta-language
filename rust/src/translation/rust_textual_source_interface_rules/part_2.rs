// Generated from complete js/src/translation/rust-textual-source-interface-rules.js. Do not edit by hand.
use super::accept_rust_scalar_source_expression;
use super::accept_rust_scalar_source_expression_field;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;

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
pub fn measure_rust_utf8_byte_length(units: &[f64]) -> self::ml::Big {
    {
        let count = self::ml::Big::from_i128(0);
        {
            let index = 0f64;
            {
                self::source_ml_source1_temporary_measure_rust_utf8_byte_length_loop1(
                    units, count, index,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_byte_length_operation() -> String {
    String::from("measureRustStringByteLength")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_byte_length_operation(expression: &str, operation: &str) -> bool {
    (expression == "math") && (operation == (self::read_rust_string_byte_length_operation()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_byte_length_argument_error(
    language: &str,
    units: bool,
    count: f64,
    spread: bool,
    kind: &str,
) -> String {
    if (language != "Rust") || units {
        String::from("the byte count needs the Rust Unicode scalar String representation")
    } else if ((count == 1f64) && !spread) && (kind == "string") {
        String::new()
    } else {
        String::from("a byte count needs one actual String operand")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_string_byte_length(target: &str, value: &str) -> String {
    if target == "JavaScript" {
        format!(
            "BigInt(encodeURIComponent({}).replace(/%[0-9A-F]{{2}}/g,\"x\").length)",
            value.to_owned()
        )
    } else if target == "Rust" {
        format!("(({}).len() as u64)", value.to_owned())
    } else if target == "Lean" {
        format!("(String.utf8ByteSize {})", value.to_owned())
    } else if target == "Rocq" {
        format!("(N.of_nat (String.length {}))", value.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_textual_source_interface_model(model: &str) -> bool {
    model == "retained textual Rust function interface 1"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_textual_source_type(kind: &str, path: &str, children: f64) -> String {
    if children == 0f64 {
        if (kind == "primitive") && (path == "str") {
            String::from("str")
        } else if (kind == "library") && (path == "std::string::String") {
            String::from("String")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_textual_source_operation(operation: &str, count: f64, result: &str) -> bool {
    if operation == (self::read_rust_string_byte_length_operation()) {
        (count == 1f64) && (result == "fixed")
    } else {
        self::accept_rust_scalar_mathematical_operation(operation, count, result)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_textual_source_binary(domain: &str, operation: &str) -> bool {
    ((domain != "string") || (operation == "eq")) || (operation == "ne")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_textual_source_input(value: &str) -> String {
    format!("::std::borrow::ToOwned::to_owned(&({}))", value.to_owned())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_textual_source_slice_argument(value: &str) -> String {
    format!("::std::string::String::as_str(&({}))", value.to_owned())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_textual_source_encoding() -> String {
    String::from("retained-textual-source-interface")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_textual_source_statement() -> String {
    String::from(
        "retained scalar calls compose with owned standard Unicode String endpoints and immutable String/str inputs; complete argument values are materialized once before reborrowing; string byte lengths retain the exact UTF-8 measure and borrowed outputs, pointer/capacity observations and arbitrary ownership remain separate obligations",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_measure_rust_utf8_byte_length_loop1(
    units: &[f64],
    mut count: self::ml::Big,
    mut index: f64,
) -> self::ml::Big {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit = self::read_required_number_fact(units, index);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (String::from(
                            "a UTF-8 byte count needs valid UTF-16 units",
                        ))
                        .encode_utf16()
                        .collect(),
                        message_present: true,
                    })
                    .raise()
                } else {
                    if unit < 128f64 {
                        {
                            let count_2 = count.add(&self::ml::Big::from_i128(1));
                            {
                                let index_2 = index + 1f64;
                                {
                                    (count, index) = (count_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if unit < 2048f64 {
                            {
                                let count_3 = count.add(&self::ml::Big::from_i128(2));
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        (count, index) = (count_3, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (55_296f64..=56_319f64).contains(&unit) {
                                if (index + 1f64) >= (units.len() as f64) {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (String::from(
                                            "a UTF-8 byte count needs Unicode scalar values",
                                        ))
                                        .encode_utf16()
                                        .collect(),
                                        message_present: true,
                                    })
                                    .raise()
                                } else {
                                    {
                                        let next =
                                            self::read_required_number_fact(units, (index + 1f64));
                                        if (!(self::ml_math::is_integer(next))
                                            || (next < 56_320f64))
                                            || (next > 57_343f64)
                                        {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("a UTF-8 byte count needs Unicode scalar values")).encode_utf16().collect(), message_present: true }).raise()
                                        } else {
                                            {
                                                let index_4 = index + 1f64;
                                                {
                                                    let count_4 =
                                                        count.add(&self::ml::Big::from_i128(4));
                                                    {
                                                        let index_5 = index_4 + 1f64;
                                                        {
                                                            (count, index) = (count_4, index_5);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if (56_320f64..=57_343f64).contains(&unit) {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (String::from(
                                        "a UTF-8 byte count needs Unicode scalar values",
                                    ))
                                    .encode_utf16()
                                    .collect(),
                                    message_present: true,
                                })
                                .raise()
                            } else {
                                {
                                    let count_5 = count.add(&self::ml::Big::from_i128(3));
                                    {
                                        let index_6 = index + 1f64;
                                        {
                                            (count, index) = (count_5, index_6);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            count.clone()
        };
    }
}
