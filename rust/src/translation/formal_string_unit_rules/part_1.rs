// Generated from complete js/src/translation/formal-string-unit-rules.js. Do not edit by hand.
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
pub fn accept_unicode_case_mapping_operation(operation: &str) -> bool {
    (operation == "toLowerCase") || (operation == "toUpperCase")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_case_encoding_statement() -> String {
    String::from(
        "Unicode 16.0.0 full locale-insensitive case mappings, contextual final sigma and preserved isolated UTF16 units; alternate Unicode/locale profiles require their own declared tables",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_unicode_case_column_mapping(target: &str, operation: &str, value: &str) -> String {
    if (self::accept_unicode_case_mapping_operation(operation)) {
        {
            let upper = if operation == "toUpperCase" {
                String::from("true")
            } else {
                String::from("false")
            };
            if target == "JavaScript" {
                format!("ml_unicode_case.mapCase({}, {upper})", value.to_owned())
            } else if target == "Rust" {
                format!(
                    "crate::ml_unicode_case::map_unicode_case_utf16_text(&({}), {upper})",
                    value.to_owned()
                )
            } else if (target == "Lean") || (target == "Rocq") {
                format!("(MLUnicodeCase.mapCase {} {upper})", value.to_owned())
            } else {
                String::new()
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_unicode_case_string_map(
    target: &str,
    operation: &str,
    units: bool,
    value: &str,
) -> String {
    if (self::accept_unicode_case_mapping_operation(operation)) {
        if target == "JavaScript" {
            {
                let input = String::from(
                    "Array.from({length:ml_text.length}, (_,ml_index)=>ml_text.charCodeAt(ml_index))",
                );
                format!(
                    "((ml_text)=>{}.map(ml_unit=>String.fromCharCode(ml_unit)).join(\"\"))({})",
                    self::render_unicode_case_column_mapping(target, operation, &(input)),
                    value.to_owned()
                )
            }
        } else if target == "Rust" {
            {
                let input_2 = if units {
                    format!(
                        "({}).0.iter().map(|ml_unit| f64::from(*ml_unit)).collect::<Vec<f64>>()",
                        value.to_owned()
                    )
                } else {
                    format!(
                        "({}).encode_utf16().map(f64::from).collect::<Vec<f64>>()",
                        value.to_owned()
                    )
                };
                {
                    let result = format!(
                        "{}.into_iter().map(|ml_unit| ml_unit as u16).collect::<Vec<u16>>()",
                        self::render_unicode_case_column_mapping(target, operation, &(input_2))
                    );
                    if units {
                        format!("crate::ml_string::Text::from_units({result})")
                    } else {
                        format!(
                            "String::from_utf16(&({result})).expect(\"Unicode case conversion preserves scalar string input\")"
                        )
                    }
                }
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
pub fn require_formal_string_unit_representation(
    kind: &str,
    operation: &str,
    receiver: &str,
) -> bool {
    if kind == "type" {
        operation == "string"
    } else if kind == "math" {
        (operation == "createJavaScriptStringFromUnits")
            || (operation == "convertJavaScriptCharacterCodesToString")
    } else if kind == "method" {
        receiver == "string"
    } else {
        ((kind == "stringPoints") || (kind == "stringMap")) || (kind == "stringTest")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_formal_string_unit_type(target: &str) -> String {
    if target == "Lean" {
        String::from("(Array Nat)")
    } else if target == "Rocq" {
        String::from("(list N)")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_array_slice(target: &str, value: &str, start: &str, end: &str) -> String {
    if target == "Lean" {
        format!(
            "(let ml_values := {}; let ml_begin := {}; let ml_end := {}; let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_begin ml_length).toUInt64.toNat; let ml_last := (MLUtf16Text.slicePosition ml_end ml_length).toUInt64.toNat; ml_values.extract ml_first ml_last)",
            value.to_owned(),
            start.to_owned(),
            end.to_owned()
        )
    } else if target == "Rocq" {
        format!(
            "(let ml_values := {} in let ml_begin := {} in let ml_end := {} in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_begin ml_length)) in let ml_last := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_end ml_length)) in List.firstn (Nat.sub ml_last ml_first) (List.skipn ml_first ml_values))",
            value.to_owned(),
            start.to_owned(),
            end.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_primitive_same_value_zero(
    target: &str,
    kind: &str,
    signed: bool,
    units: bool,
    left: &str,
    right: &str,
) -> String {
    if kind == "unit" {
        String::from("true")
    } else if (kind == "string") && units {
        self::render_formal_string_unit_comparison(target, "eq", left, right)
    } else if (((((kind != "bool") && (kind != "float")) && (kind != "int")) && (kind != "nat"))
        && (kind != "fixed"))
        && (kind != "string")
    {
        String::new()
    } else if target == "Lean" {
        if kind == "float" {
            format!(
                "(({} == {}) || (({}).isNaN && ({}).isNaN))",
                left.to_owned(),
                right.to_owned(),
                left.to_owned(),
                right.to_owned()
            )
        } else {
            format!("({} == {})", left.to_owned(), right.to_owned())
        }
    } else if target == "Rocq" {
        if kind == "float" {
            format!(
                "(orb (PrimFloat.eqb {} {}) (andb (PrimFloat.is_nan {}) (PrimFloat.is_nan {})))",
                left.to_owned(),
                right.to_owned(),
                left.to_owned(),
                right.to_owned()
            )
        } else {
            {
                let module = if kind == "bool" {
                    String::from("Bool")
                } else if kind == "string" {
                    String::from("String")
                } else if (kind == "int") || ((kind == "fixed") && signed) {
                    String::from("Z")
                } else {
                    String::from("N")
                };
                format!("({module}.eqb {} {})", left.to_owned(), right.to_owned())
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_array_inclusion(
    target: &str,
    value: &str,
    search: &str,
    position: &str,
    equality: &str,
) -> String {
    if target == "Lean" {
        format!(
            "(let ml_values := {}; let ml_search := {}; let ml_start := {}; let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_start ml_length).toUInt64.toNat; (ml_values.extract ml_first ml_values.size).any (fun ml_candidate => {}))",
            value.to_owned(),
            search.to_owned(),
            position.to_owned(),
            equality.to_owned()
        )
    } else if target == "Rocq" {
        format!(
            "(let ml_values := {} in let ml_search := {} in let ml_start := {} in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_start ml_length)) in List.existsb (fun ml_candidate => {}) (List.skipn ml_first ml_values))",
            value.to_owned(),
            search.to_owned(),
            position.to_owned(),
            equality.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_unit_literal(target: &str, units: &[String]) -> String {
    if target == "Lean" {
        format!(
            "(#[{}] : Array Nat)",
            units.join((String::from(", ")).as_str())
        )
    } else if target == "Rocq" {
        format!(
            "({}{}(@nil N))",
            ({
                let ml_values1 = units.to_vec();
                let ml_callback1 = std::rc::Rc::new(move |unit: String| -> String {
                    format!("{}{}", unit, (String::from("%N")))
                });
                ml_values1
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                    .collect::<Vec<_>>()
            })
            .join((String::from(" :: ")).as_str()),
            if {
                let ml_value = units.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            } {
                String::from(" :: ")
            } else {
                String::new()
            }
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_unit_numbers(target: &str, value: &str) -> String {
    if target == "Lean" {
        format!("(({}).map Float.ofNat)", value.to_owned())
    } else if target == "Rocq" {
        format!(
            "(List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) {})",
            value.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_units_from_numbers(target: &str, value: &str) -> String {
    if target == "Lean" {
        format!(
            "(({}).map (fun ml_unit => ml_unit.toUInt64.toNat))",
            value.to_owned()
        )
    } else if target == "Rocq" {
        format!(
            "(List.map (fun ml_unit => Z.to_N (ml_float_index ml_unit)) {})",
            value.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_units_from_scalar(target: &str, value: &str) -> String {
    if target == "Lean" {
        format!(
            "(String.foldl (fun (ml_units : Array Nat) ml_character => if ml_character.toNat < 65536 then ml_units.push ml_character.toNat else (ml_units.push (55296 + (ml_character.toNat - 65536) / 1024)).push (56320 + (ml_character.toNat - 65536) % 1024)) #[] {})",
            value.to_owned()
        )
    } else if target == "Rocq" {
        {
            let bytes = format!(
                "((fix ml_text_bytes (ml_text : string) : list float := match ml_text with EmptyString => nil | String ml_character ml_rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii ml_character)))) (ml_text_bytes ml_rest) end) {})",
                value.to_owned()
            );
            self::render_formal_string_units_from_numbers(
                target,
                &(format!("(MLUtf16Text.fromUtf8 {bytes})")),
            )
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_scalar_display(target: &str, value: &str) -> String {
    {
        let numbers = self::render_formal_string_unit_numbers(target, value);
        if target == "Lean" {
            format!(
                "(String.ofList ((MLUtf16Text.displayPoints {numbers}).toList.map (fun ml_point => Char.ofNat ml_point.toUInt64.toNat)))"
            )
        } else if target == "Rocq" {
            format!(
                "((fix ml_display_bytes (ml_bytes : list float) : string := match ml_bytes with nil => EmptyString | cons ml_byte ml_rest => String (Ascii.ascii_of_N (Z.to_N (ml_float_index ml_byte))) (ml_display_bytes ml_rest) end) (MLUtf16Text.displayUtf8 {numbers}))"
            )
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_unit_comparison(
    target: &str,
    operation: &str,
    left: &str,
    right: &str,
) -> String {
    {
        let value = format!(
            "(MLUtf16Text.compare {} {})",
            self::render_formal_string_unit_numbers(target, left),
            self::render_formal_string_unit_numbers(target, right)
        );
        if target == "Lean" {
            if operation == "eq" {
                format!("({value} == (0 : Float))")
            } else if operation == "ne" {
                format!("({value} != (0 : Float))")
            } else if operation == "lt" {
                format!("(decide ({value} < (0 : Float)))")
            } else if operation == "le" {
                format!("(decide ({value} <= (0 : Float)))")
            } else if operation == "gt" {
                format!("(decide ({value} > (0 : Float)))")
            } else if operation == "ge" {
                format!("(decide ({value} >= (0 : Float)))")
            } else {
                String::new()
            }
        } else if target == "Rocq" {
            if operation == "eq" {
                format!("(PrimFloat.eqb {value} 0%float)")
            } else if operation == "ne" {
                format!("(negb (PrimFloat.eqb {value} 0%float))")
            } else if operation == "lt" {
                format!("(PrimFloat.ltb {value} 0%float)")
            } else if operation == "le" {
                format!("(PrimFloat.leb {value} 0%float)")
            } else if operation == "gt" {
                format!("(PrimFloat.ltb 0%float {value})")
            } else if operation == "ge" {
                format!("(PrimFloat.leb 0%float {value})")
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_string_unit_operation(
    target: &str,
    operation: &str,
    values: &[String],
) -> String {
    if operation == "join" {
        {
            let value = (self::ml_array::optional_at(values, self::ml_array::number_index(0f64)))
                .unwrap_or_else(|| String::new());
            {
                let numbers = self::render_formal_string_unit_numbers(target, "ml_text");
                {
                    let texts = if target == "Lean" {
                        format!("(({value}).map (fun ml_text => {numbers}))")
                    } else {
                        format!("(List.map (fun ml_text => {numbers}) {value})")
                    };
                    self::render_formal_string_units_from_numbers(
                        target,
                        &(format!(
                            "(MLUtf16Text.join {texts} {})",
                            self::render_formal_string_unit_numbers(
                                target,
                                &((self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(1f64)
                                ))
                                .unwrap_or_else(|| String::new()))
                            )
                        )),
                    )
                }
            }
        }
    } else {
        {
            let text = self::render_formal_string_unit_numbers(
                target,
                &((self::ml_array::optional_at(values, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| String::new())),
            );
            if (operation == "toLowerCase") || (operation == "toUpperCase") {
                self::render_formal_string_units_from_numbers(
                    target,
                    &(self::render_unicode_case_column_mapping(target, operation, &(text))),
                )
            } else {
                {
                    let position =
                        (self::ml_array::optional_at(values, self::ml_array::number_index(1f64)))
                            .unwrap_or_else(|| String::new());
                    if ((operation == "slice") || (operation == "substring"))
                        || (operation == "charAt")
                    {
                        {
                            let name = if operation == "charAt" {
                                String::from("character")
                            } else {
                                operation.to_owned()
                            };
                            self::render_formal_string_units_from_numbers(
                                target,
                                &(format!(
                                    "(MLUtf16Text.{name} {text} {position}{})",
                                    if operation == "charAt" {
                                        String::new()
                                    } else {
                                        format!(
                                            "{}{}",
                                            (String::from(" ")),
                                            ((self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index(2f64)
                                            ))
                                            .unwrap_or_else(|| String::new()))
                                        )
                                    }
                                )),
                            )
                        }
                    } else if operation == "at" {
                        {
                            let result =
                                format!("(MLUtf16Text.relativeCharacter {text} {position})");
                            if target == "Lean" {
                                format!(
                                    "(({result})[0]?.map (fun ml_units => {}))",
                                    self::render_formal_string_units_from_numbers(
                                        target, "ml_units"
                                    )
                                )
                            } else {
                                format!(
                                    "(match {result} with nil => None | cons ml_units _ => Some {} end)",
                                    self::render_formal_string_units_from_numbers(
                                        target, "ml_units"
                                    )
                                )
                            }
                        }
                    } else if operation == "codePointAt" {
                        {
                            let result_2 = format!("(MLUtf16Text.codePoint {text} {position})");
                            if target == "Lean" {
                                format!("({result_2})[0]?")
                            } else {
                                format!(
                                    "(match {result_2} with nil => None | cons ml_point _ => Some ml_point end)"
                                )
                            }
                        }
                    } else if (operation == "indexOf") || (operation == "lastIndexOf") {
                        format!(
                            "(MLUtf16Text.{} {text} {} {})",
                            if operation == "indexOf" {
                                String::from("index")
                            } else {
                                String::from("lastIndex")
                            },
                            self::render_formal_string_unit_numbers(
                                target,
                                &((self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(1f64)
                                ))
                                .unwrap_or_else(|| String::new()))
                            ),
                            (self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(2f64)
                            ))
                            .unwrap_or_else(|| String::new())
                        )
                    } else if operation == "points" {
                        {
                            let result_3 = format!("(MLUtf16Text.points {text})");
                            if target == "Lean" {
                                format!(
                                    "(({result_3}).map (fun ml_units => {}))",
                                    self::render_formal_string_units_from_numbers(
                                        target, "ml_units"
                                    )
                                )
                            } else {
                                format!(
                                    "(List.map (fun ml_units => {}) {result_3})",
                                    self::render_formal_string_units_from_numbers(
                                        target, "ml_units"
                                    )
                                )
                            }
                        }
                    } else if ((operation == "trim") || (operation == "trimStart"))
                        || (operation == "trimEnd")
                    {
                        self::render_formal_string_units_from_numbers(
                            target,
                            &(format!(
                                "(MLUtf16Text.trim {text} {} {})",
                                if operation == "trimEnd" {
                                    String::from("false")
                                } else {
                                    String::from("true")
                                },
                                if operation == "trimStart" {
                                    String::from("false")
                                } else {
                                    String::from("true")
                                }
                            )),
                        )
                    } else {
                        {
                            let name_2 = if operation == "startsWith" {
                                String::from("starts")
                            } else if operation == "endsWith" {
                                String::from("ends")
                            } else if operation == "includes" {
                                String::from("includes")
                            } else {
                                String::new()
                            };
                            if name_2.is_empty() {
                                String::new()
                            } else {
                                format!(
                                    "(MLUtf16Text.{name_2} {text} {})",
                                    self::render_formal_string_unit_numbers(
                                        target,
                                        &((self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(1f64)
                                        ))
                                        .unwrap_or_else(|| String::new()))
                                    )
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
