// Generated from complete js/src/translation/rust-saturating-multiplication-rules.js. Do not edit by hand.
/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "Rust source function signature 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_wrapper(wrapper: &str) -> bool {
    (wrapper == "reference") || (wrapper == "box")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_owner_kind(kind: &str) -> bool {
    (((((kind == "primitive") || (kind == "declaration")) || (kind == "library"))
        || (kind == "tuple"))
        || (kind == "unit"))
        || (kind == "slice")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_catalog_owner(kind: &str) -> bool {
    (((kind == "data") || (kind == "module")) || (kind == "externalCrate")) || (kind == "ctor")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_primitive(
    kind: &str,
    path: &str,
    bits: &str,
    signed: bool,
) -> bool {
    if path == "f64" {
        kind == "float"
    } else if path == "bool" {
        kind == "bool"
    } else if path == "str" {
        kind == "string"
    } else if path == "i8" {
        ((kind == "fixed") && (bits == "8")) && (signed == (true))
    } else if path == "u8" {
        ((kind == "fixed") && (bits == "8")) && (signed == (false))
    } else if path == "i16" {
        ((kind == "fixed") && (bits == "16")) && (signed == (true))
    } else if path == "u16" {
        ((kind == "fixed") && (bits == "16")) && (signed == (false))
    } else if path == "i32" {
        ((kind == "fixed") && (bits == "32")) && (signed == (true))
    } else if path == "u32" {
        ((kind == "fixed") && (bits == "32")) && (signed == (false))
    } else if path == "i64" {
        ((kind == "fixed") && (bits == "64")) && (signed == (true))
    } else if path == "u64" {
        ((kind == "fixed") && (bits == "64")) && (signed == (false))
    } else if path == "i128" {
        ((kind == "fixed") && (bits == "128")) && (signed == (true))
    } else if path == "u128" {
        ((kind == "fixed") && (bits == "128")) && (signed == (false))
    } else if path == "usize" {
        ((kind == "fixed") && (bits == "64")) && (signed == (false))
    } else if path == "isize" {
        ((kind == "fixed") && (bits == "64")) && (signed == (true))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_call_role(
    path: &str,
    ordinary: bool,
    shadowed: bool,
) -> String {
    if ordinary && !shadowed {
        if path == "std::cmp::min" {
            String::from("min")
        } else if path == "std::cmp::max" {
            String::from("max")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_comparison(method: &str, ordinary: bool) -> String {
    if ordinary {
        if method == "min" {
            String::from("le")
        } else if method == "max" {
            String::from("gt")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_condition(method: &str, kind: &str) -> String {
    if kind == "bool" {
        if method == "min" {
            String::from("not")
        } else if method == "max" {
            String::from("identity")
        } else {
            String::new()
        }
    } else if kind == "fixed" {
        self::read_rust_primitive_selection_comparison(method, true)
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_receiver(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((((kind == "fixed") || (kind == "bool")) && (owner == "primitive")) && (children == 0f64))
        && proven)
        && (layers == 0f64))
        && self::accept_rust_source_signature_primitive(kind, path, bits, signed)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_argument(
    expected: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    actual: &str,
    actual_bits: &str,
    actual_signed: bool,
    actual_owner: &str,
    layers: f64,
    proven: bool,
    known: bool,
    literal: bool,
) -> bool {
    if layers == 0f64 {
        if ((literal && (expected == "fixed")) && ((actual).is_empty()))
            && ((actual_owner).is_empty())
        {
            true
        } else {
            ((((known && proven) && (expected == actual)) && (bits == actual_bits))
                && (signed == actual_signed))
                && (owner == actual_owner)
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_saturating_multiplication_method(method: &str, ordinary: bool) -> bool {
    ordinary && (method == "saturating_mul")
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_saturating_multiplication_receiver(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (kind == "fixed")
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_saturating_multiplication(signed: bool) -> Vec<String> {
    {
        let product = vec![
            String::from("operand"),
            String::from("right"),
            String::from("mul"),
        ];
        {
            let positive_positive = {
                let source_ml_concat_value1 = vec![
                    String::from("operand"),
                    String::from("maximum"),
                    String::from("right"),
                    String::from("div"),
                    String::from("gt"),
                    String::from("maximum"),
                ];
                {
                    let source_ml_concat_value2 = product.clone();
                    {
                        let source_ml_concat_value3 = vec![String::from("if")];
                        self::ml_array::append(
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<String>::new(),
                                    source_ml_concat_value1,
                                ),
                                source_ml_concat_value2,
                            ),
                            source_ml_concat_value3,
                        )
                    }
                }
            };
            if signed {
                {
                    let positive_negative = {
                        let source_ml_concat_value7 = vec![
                            String::from("right"),
                            String::from("minimum"),
                            String::from("operand"),
                            String::from("div"),
                            String::from("lt"),
                            String::from("minimum"),
                        ];
                        {
                            let source_ml_concat_value8 = product.clone();
                            {
                                let source_ml_concat_value9 = vec![String::from("if")];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value7,
                                        ),
                                        source_ml_concat_value8,
                                    ),
                                    source_ml_concat_value9,
                                )
                            }
                        }
                    };
                    {
                        let negative_positive = {
                            let source_ml_concat_value10 = vec![
                                String::from("operand"),
                                String::from("minimum"),
                                String::from("right"),
                                String::from("div"),
                                String::from("lt"),
                                String::from("minimum"),
                            ];
                            {
                                let source_ml_concat_value11 = product.clone();
                                {
                                    let source_ml_concat_value12 = vec![String::from("if")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_concat_value10,
                                            ),
                                            source_ml_concat_value11,
                                        ),
                                        source_ml_concat_value12,
                                    )
                                }
                            }
                        };
                        {
                            let negative_negative = {
                                let source_ml_concat_value13 = vec![
                                    String::from("operand"),
                                    String::from("maximum"),
                                    String::from("right"),
                                    String::from("div"),
                                    String::from("lt"),
                                    String::from("maximum"),
                                ];
                                {
                                    let source_ml_concat_value14 = product;
                                    {
                                        let source_ml_concat_value15 = vec![String::from("if")];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_concat_value13,
                                                ),
                                                source_ml_concat_value14,
                                            ),
                                            source_ml_concat_value15,
                                        )
                                    }
                                }
                            };
                            {
                                let signs = {
                                    let source_ml_concat_value16 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("gt"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("gt"),
                                    ];
                                    {
                                        let source_ml_concat_value17 = positive_positive;
                                        {
                                            let source_ml_concat_value18 = positive_negative;
                                            {
                                                let source_ml_concat_value19 = vec![
                                                    String::from("if"),
                                                    String::from("right"),
                                                    String::from("zero"),
                                                    String::from("gt"),
                                                ];
                                                {
                                                    let source_ml_concat_value20 =
                                                        negative_positive;
                                                    {
                                                        let source_ml_concat_value21 =
                                                            negative_negative;
                                                        {
                                                            let source_ml_concat_value22 = vec![
                                                                String::from("if"),
                                                                String::from("if"),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_concat_value16), source_ml_concat_value17), source_ml_concat_value18), source_ml_concat_value19), source_ml_concat_value20), source_ml_concat_value21), source_ml_concat_value22)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_concat_value23 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("eq"),
                                        String::from("zero"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("eq"),
                                        String::from("zero"),
                                    ];
                                    {
                                        let source_ml_concat_value24 = signs;
                                        {
                                            let source_ml_concat_value25 =
                                                vec![String::from("if"), String::from("if")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_concat_value23,
                                                    ),
                                                    source_ml_concat_value24,
                                                ),
                                                source_ml_concat_value25,
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let source_ml_concat_value4 = vec![
                        String::from("operand"),
                        String::from("zero"),
                        String::from("eq"),
                        String::from("zero"),
                        String::from("right"),
                        String::from("zero"),
                        String::from("eq"),
                        String::from("zero"),
                    ];
                    {
                        let source_ml_concat_value5 = positive_positive;
                        {
                            let source_ml_concat_value6 =
                                vec![String::from("if"), String::from("if")];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_concat_value4,
                                    ),
                                    source_ml_concat_value5,
                                ),
                                source_ml_concat_value6,
                            )
                        }
                    }
                }
            }
        }
    }
}
