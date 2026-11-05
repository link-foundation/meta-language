// Generated from complete js/src/translation/rust-saturating-integer-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::source_ml_source1_temporary_read_rust_integer_bound_predecessor_loop2;
use super::source_ml_source1_temporary_read_rust_integer_bound_predecessor_loop3;
use super::source_ml_source1_temporary_wrap_fixed_integer_bits_loop1;

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

#[path = "ml_big_integer_conversion.rs"]
pub mod ml_big_integer_conversion;

#[path = "ml_utf16_text.rs"]
pub mod ml_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_model(model: &self::ml_string::Text) -> bool {
    ((*model) == (self::ml_string::Text::from_units(vec![])))
        || ((*model)
            == (self::ml_string::Text::from_units(vec![
                82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116,
                105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 49,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_wrapper(wrapper: &self::ml_string::Text) -> bool {
    ((*wrapper)
        == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101])))
        || ((*wrapper) == (self::ml_string::Text::from_units(vec![98, 111, 120])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_owner_kind(kind: &self::ml_string::Text) -> bool {
    ((((((*kind)
        == (self::ml_string::Text::from_units(vec![
            112, 114, 105, 109, 105, 116, 105, 118, 101,
        ])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
            ]))))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_catalog_owner(kind: &self::ml_string::Text) -> bool {
    ((((*kind) == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101]))))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                101, 120, 116, 101, 114, 110, 97, 108, 67, 114, 97, 116, 101,
            ]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_primitive(
    kind: &self::ml_string::Text,
    path: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
) -> bool {
    if (*path) == (self::ml_string::Text::from_units(vec![102, 54, 52])) {
        (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        (*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![115, 116, 114])) {
        (*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![56]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![56]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 49, 54])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 54]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 49, 54])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 54]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 51, 50])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 51, 50])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 54, 52])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 54, 52])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 50, 56]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 49, 50, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 50, 56]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (true))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_call_role(
    path: &self::ml_string::Text,
    ordinary: bool,
    shadowed: bool,
) -> self::ml_string::Text {
    if ordinary && !shadowed {
        if (*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 109, 105, 110,
            ]))
        {
            self::ml_string::Text::from_units(vec![109, 105, 110])
        } else if (*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 109, 97, 120,
            ]))
        {
            self::ml_string::Text::from_units(vec![109, 97, 120])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_comparison(
    method: &self::ml_string::Text,
    ordinary: bool,
) -> self::ml_string::Text {
    if ordinary {
        if (*method) == (self::ml_string::Text::from_units(vec![109, 105, 110])) {
            self::ml_string::Text::from_units(vec![108, 101])
        } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 120])) {
            self::ml_string::Text::from_units(vec![103, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_condition(
    method: &self::ml_string::Text,
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        if (*method) == (self::ml_string::Text::from_units(vec![109, 105, 110])) {
            self::ml_string::Text::from_units(vec![110, 111, 116])
        } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 120])) {
            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else if (*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])) {
        self::read_rust_primitive_selection_comparison(method, true)
    } else {
        self::ml_string::Text::from_units(vec![])
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
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (((((((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101,
            ]))))
        && (children == 0f64))
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
    expected: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    actual: &self::ml_string::Text,
    actual_bits: &self::ml_string::Text,
    actual_signed: bool,
    actual_owner: &self::ml_string::Text,
    layers: f64,
    proven: bool,
    known: bool,
    literal: bool,
) -> bool {
    if layers == 0f64 {
        if ((literal
            && ((*expected) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
            && ((*actual) == (self::ml_string::Text::from_units(vec![]))))
            && ((*actual_owner) == (self::ml_string::Text::from_units(vec![])))
        {
            true
        } else {
            ((((known && proven) && ((*expected) == (*actual))) && ((*bits) == (*actual_bits)))
                && (signed == actual_signed))
                && ((*owner) == (*actual_owner))
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_fixed_integer_cast_width(bits: f64) -> bool {
    ((((bits == 8f64) || (bits == 16f64)) || (bits == 32f64)) || (bits == 64f64))
        || (bits == 128f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_modulus(bits: f64) -> self::ml_string::Text {
    if bits == 8f64 {
        self::ml_string::Text::from_units(vec![50, 53, 54])
    } else if bits == 16f64 {
        self::ml_string::Text::from_units(vec![54, 53, 53, 51, 54])
    } else if bits == 32f64 {
        self::ml_string::Text::from_units(vec![52, 50, 57, 52, 57, 54, 55, 50, 57, 54])
    } else if bits == 64f64 {
        self::ml_string::Text::from_units(vec![
            49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 54,
        ])
    } else if bits == 128f64 {
        self::ml_string::Text::from_units(vec![
            51, 52, 48, 50, 56, 50, 51, 54, 54, 57, 50, 48, 57, 51, 56, 52, 54, 51, 52, 54, 51, 51,
            55, 52, 54, 48, 55, 52, 51, 49, 55, 54, 56, 50, 49, 49, 52, 53, 54,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_signed_limit(bits: f64) -> self::ml_string::Text {
    if bits == 8f64 {
        self::ml_string::Text::from_units(vec![49, 50, 56])
    } else if bits == 16f64 {
        self::ml_string::Text::from_units(vec![51, 50, 55, 54, 56])
    } else if bits == 32f64 {
        self::ml_string::Text::from_units(vec![50, 49, 52, 55, 52, 56, 51, 54, 52, 56])
    } else if bits == 64f64 {
        self::ml_string::Text::from_units(vec![
            57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56,
        ])
    } else if bits == 128f64 {
        self::ml_string::Text::from_units(vec![
            49, 55, 48, 49, 52, 49, 49, 56, 51, 52, 54, 48, 52, 54, 57, 50, 51, 49, 55, 51, 49, 54,
            56, 55, 51, 48, 51, 55, 49, 53, 56, 56, 52, 49, 48, 53, 55, 50, 56,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_flavor(
    language: &self::ml_string::Text,
    widening: bool,
    flavor: &self::ml_string::Text,
) -> self::ml_string::Text {
    if widening {
        self::ml_string::Text::from_units(vec![101, 120, 97, 99, 116])
    } else if ((*language) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])))
        || ((*flavor)
            == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 105, 110, 103])))
    {
        self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 105, 110, 103])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        70, 105, 120, 101, 100, 32, 119, 105, 100, 116, 104, 32, 105, 110, 116, 101, 103, 101, 114,
        32, 99, 97, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 108, 111, 119, 32, 98, 105,
        116, 115, 32, 97, 116, 32, 116, 104, 101, 32, 116, 97, 114, 103, 101, 116, 32, 119, 105,
        100, 116, 104, 32, 97, 110, 100, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 32, 105,
        116, 115, 32, 115, 105, 103, 110, 32, 98, 105, 116, 32, 102, 111, 114, 32, 115, 105, 103,
        110, 101, 100, 32, 116, 97, 114, 103, 101, 116, 115, 44, 32, 119, 105, 116, 104, 111, 117,
        116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 111, 117, 110, 100, 105, 110, 103, 32,
        116, 104, 114, 111, 117, 103, 104, 32, 98, 105, 110, 97, 114, 121, 54, 52, 46,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn wrap_fixed_integer_bits(value: self::ml::Big, bits: f64, signed: bool) -> self::ml::Big {
    if (self::accept_fixed_integer_cast_width(bits)) {
        {
            let modulus = self::ml::Big::from_i128(1);
            {
                let bit = 0f64;
                {
                    let modulus_2 = self::source_ml_source1_temporary_wrap_fixed_integer_bits_loop1(
                        bits, modulus, bit,
                    );
                    {
                        let residue = {
                            let source_ml_source2_integer_division1 =
                                self::ml_big_integer_conversion::divide_java_script_big_integer(
                                    value.clone(),
                                    modulus_2.clone(),
                                );
                            if (source_ml_source2_integer_division1.len() as f64) == 2f64 {
                                (self::ml_array::optional_at(
                                    &source_ml_source2_integer_division1,
                                    self::ml_array::number_index(1f64),
                                ))
                                .unwrap_or_else(|| self::ml::Big::from_i128(0))
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("RangeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32,
                                        122, 101, 114, 111,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        {
                            let residue_3 = if residue < (self::ml::Big::from_i128(0)) {
                                { residue.add(&modulus_2) }
                            } else {
                                residue.clone()
                            };
                            if signed
                                && (residue_3
                                    >= (self::ml::divide(
                                        &modulus_2,
                                        &self::ml::Big::from_i128(2),
                                        "trunc",
                                        Some("Division by zero"),
                                        false,
                                    )))
                            {
                                residue_3.sub(&modulus_2)
                            } else {
                                residue_3.clone()
                            }
                        }
                    }
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 102, 105, 120, 101, 100,
                32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 97, 115, 116, 32, 119, 105, 100,
                116, 104,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_fixed_integer_cast_expression(bits: f64, signed: bool) -> Vec<self::ml_string::Text> {
    {
        let modulus = self::read_fixed_integer_cast_modulus(bits);
        if modulus == (self::ml_string::Text::from_units(vec![])) {
            Vec::<self::ml_string::Text>::new()
        } else {
            vec![
                self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]),
                modulus,
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]),
                self::ml_string::Text::from_units(vec![48]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]),
                self::ml_string::Text::from_units(vec![48]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]),
                self::ml_string::Text::from_units(vec![114, 101, 109]),
                self::ml_string::Text::from_units(vec![51]),
                self::ml_string::Text::from_units(vec![49]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]),
                self::ml_string::Text::from_units(vec![49]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]),
                self::ml_string::Text::from_units(vec![108, 116]),
                self::ml_string::Text::from_units(vec![53]),
                self::ml_string::Text::from_units(vec![50]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]),
                self::ml_string::Text::from_units(vec![97, 100, 100]),
                self::ml_string::Text::from_units(vec![53]),
                self::ml_string::Text::from_units(vec![49]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108,
                ]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![54]),
                self::ml_string::Text::from_units(vec![55]),
                self::ml_string::Text::from_units(vec![53]),
                self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]),
                self::ml_string::Text::from_units(vec![50]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]),
                self::read_fixed_integer_cast_signed_limit(bits),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]),
                self::ml_string::Text::from_units(vec![103, 101]),
                self::ml_string::Text::from_units(vec![57]),
                self::ml_string::Text::from_units(vec![49, 48]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]),
                self::ml_string::Text::from_units(vec![115, 117, 98]),
                self::ml_string::Text::from_units(vec![57]),
                self::ml_string::Text::from_units(vec![49]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108,
                ]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![49, 49]),
                self::ml_string::Text::from_units(vec![49, 50]),
                self::ml_string::Text::from_units(vec![57]),
                self::ml_string::Text::from_units(vec![108, 101, 116]),
                self::ml_string::Text::from_units(vec![50]),
                self::ml_string::Text::from_units(vec![56]),
                if signed {
                    self::ml_string::Text::from_units(vec![49, 51])
                } else {
                    self::ml_string::Text::from_units(vec![57])
                },
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![108, 101, 116]),
                self::ml_string::Text::from_units(vec![49]),
                self::ml_string::Text::from_units(vec![52]),
                self::ml_string::Text::from_units(vec![49, 52]),
                self::ml_string::Text::from_units(vec![]),
                self::ml_string::Text::from_units(vec![108, 101, 116]),
                self::ml_string::Text::from_units(vec![48]),
                self::ml_string::Text::from_units(vec![48]),
                self::ml_string::Text::from_units(vec![49, 53]),
                self::ml_string::Text::from_units(vec![]),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_wrapping_fixed_integer_cast(
    target: &self::ml_string::Text,
    argument: &self::ml_string::Text,
    bits: f64,
    signed: bool,
    source_signed: bool,
) -> self::ml_string::Text {
    {
        let modulus = self::read_fixed_integer_cast_modulus(bits);
        {
            let limit = self::read_fixed_integer_cast_signed_limit(bits);
            if modulus == (self::ml_string::Text::from_units(vec![])) {
                self::ml_string::Text::from_units(vec![])
            } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
                (((((self::ml_string::Text::from_units(vec![40, 40])).concat(argument)).concat(
                    &self::ml_string::Text::from_units(vec![41, 32, 97, 115, 32]),
                ))
                .concat(&if signed {
                    self::ml_string::Text::from_units(vec![105])
                } else {
                    self::ml_string::Text::from_units(vec![117])
                }))
                .concat(&self::ml_string::Text::from_text(
                    &self::ml_number::js_number(bits),
                )))
                .concat(&self::ml_string::Text::from_units(vec![41]))
            } else if (*target)
                == (self::ml_string::Text::from_units(vec![
                    74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                ]))
            {
                ((((((self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46]))
                    .concat(&if signed {
                        self::ml_string::Text::from_units(vec![97, 115, 73, 110, 116, 78])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 115, 85, 105, 110, 116, 78])
                    }))
                .concat(&self::ml_string::Text::from_units(vec![40])))
                .concat(&self::ml_string::Text::from_text(
                    &self::ml_number::js_number(bits),
                )))
                .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                .concat(argument))
                .concat(&self::ml_string::Text::from_units(vec![41]))
            } else if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
                {
                    let value = if source_signed {
                        ((self::ml_string::Text::from_units(vec![40])).concat(argument))
                            .concat(&self::ml_string::Text::from_units(vec![41]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![
                            40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40,
                        ]))
                        .concat(argument))
                        .concat(&self::ml_string::Text::from_units(vec![41, 41]))
                    };
                    {
                        let residue = ((((self::ml_string::Text::from_units(vec![
                            40, 73, 110, 116, 46, 101, 109, 111, 100, 32,
                        ]))
                        .concat(&value))
                        .concat(&self::ml_string::Text::from_units(vec![32, 40])))
                        .concat(&modulus))
                        .concat(&self::ml_string::Text::from_units(vec![
                            32, 58, 32, 73, 110, 116, 41, 41,
                        ]));
                        if signed {
                            ((((((self::ml_string::Text::from_units(vec![
                                40, 108, 101, 116, 32, 109, 108, 95, 99, 97, 115, 116, 95, 118, 97,
                                108, 117, 101, 32, 58, 32, 73, 110, 116, 32, 58, 61, 32,
                            ]))
                            .concat(&residue))
                            .concat(&self::ml_string::Text::from_units(vec![
                                59, 32, 105, 102, 32, 109, 108, 95, 99, 97, 115, 116, 95, 118, 97,
                                108, 117, 101, 32, 60, 32, 40,
                            ])))
                            .concat(&limit))
                            .concat(&self::ml_string::Text::from_units(vec![
                                32, 58, 32, 73, 110, 116, 41, 32, 116, 104, 101, 110, 32, 109, 108,
                                95, 99, 97, 115, 116, 95, 118, 97, 108, 117, 101, 32, 101, 108,
                                115, 101, 32, 109, 108, 95, 99, 97, 115, 116, 95, 118, 97, 108,
                                117, 101, 32, 45, 32, 40,
                            ])))
                            .concat(&modulus))
                            .concat(&self::ml_string::Text::from_units(vec![
                                32, 58, 32, 73, 110, 116, 41, 41,
                            ]))
                        } else {
                            ((self::ml_string::Text::from_units(vec![
                                40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32,
                            ]))
                            .concat(&residue))
                            .concat(&self::ml_string::Text::from_units(vec![41]))
                        }
                    }
                }
            } else if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
                {
                    let value_2 = if source_signed {
                        ((self::ml_string::Text::from_units(vec![40])).concat(argument))
                            .concat(&self::ml_string::Text::from_units(vec![41]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![
                            40, 90, 46, 111, 102, 95, 78, 32, 40,
                        ]))
                        .concat(argument))
                        .concat(&self::ml_string::Text::from_units(vec![41, 41]))
                    };
                    {
                        let residue_2 = ((((self::ml_string::Text::from_units(vec![
                            40, 90, 46, 109, 111, 100, 117, 108, 111, 32,
                        ]))
                        .concat(&value_2))
                        .concat(&self::ml_string::Text::from_units(vec![32, 40])))
                        .concat(&modulus))
                        .concat(&self::ml_string::Text::from_units(vec![37, 90, 41, 41]));
                        if signed {
                            ((((((self::ml_string::Text::from_units(vec![
                                40, 108, 101, 116, 32, 109, 108, 95, 99, 97, 115, 116, 95, 118, 97,
                                108, 117, 101, 32, 58, 32, 90, 32, 58, 61, 32,
                            ]))
                            .concat(&residue_2))
                            .concat(&self::ml_string::Text::from_units(vec![
                                32, 105, 110, 32, 105, 102, 32, 90, 46, 108, 116, 98, 32, 109, 108,
                                95, 99, 97, 115, 116, 95, 118, 97, 108, 117, 101, 32, 40,
                            ])))
                            .concat(&limit))
                            .concat(&self::ml_string::Text::from_units(vec![
                                37, 90, 41, 32, 116, 104, 101, 110, 32, 109, 108, 95, 99, 97, 115,
                                116, 95, 118, 97, 108, 117, 101, 32, 101, 108, 115, 101, 32, 90,
                                46, 115, 117, 98, 32, 109, 108, 95, 99, 97, 115, 116, 95, 118, 97,
                                108, 117, 101, 32, 40,
                            ])))
                            .concat(&modulus))
                            .concat(&self::ml_string::Text::from_units(vec![37, 90, 41, 41]))
                        } else {
                            ((self::ml_string::Text::from_units(vec![
                                40, 90, 46, 116, 111, 95, 78, 32,
                            ]))
                            .concat(&residue_2))
                            .concat(&self::ml_string::Text::from_units(vec![41]))
                        }
                    }
                }
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_saturating_integer_arity(method: &self::ml_string::Text, ordinary: bool) -> f64 {
    if ordinary {
        if ((*method)
            == (self::ml_string::Text::from_units(vec![
                115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 100, 100,
            ])))
            || ((*method)
                == (self::ml_string::Text::from_units(vec![
                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98,
                ])))
        {
            1f64
        } else if ((*method)
            == (self::ml_string::Text::from_units(vec![
                115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 110, 101, 103,
            ])))
            || ((*method)
                == (self::ml_string::Text::from_units(vec![
                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 98, 115,
                ])))
        {
            0f64
        } else {
            -1f64
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_saturating_integer_receiver(
    method: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((self::read_rust_saturating_integer_arity(method, true)) >= 0f64)
        && ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
        && ((signed
            || ((*method)
                == (self::ml_string::Text::from_units(vec![
                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 100, 100,
                ]))))
            || ((*method)
                == (self::ml_string::Text::from_units(vec![
                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98,
                ])))))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_bound_predecessor(
    source: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*source) == (self::ml_string::Text::from_units(vec![])) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let end = source.length();
            {
                let end_2 =
                    self::source_ml_source1_temporary_read_rust_integer_bound_predecessor_loop2(
                        source, end,
                    );
                if end_2 == 0f64 {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    {
                        let result = ({
                            let source_ml_source2_string_range_operand2 = source.clone();
                            {
                                let source_ml_source2_string_range_operand3 = 0f64;
                                {
                                    let source_ml_source2_string_range_operand4 = end_2 - 1f64;
                                    {
                                        let source_ml_source2_string_range_position5 = source_ml_source2_string_range_operand3;
                                        {
                                            let source_ml_source2_string_range_position6 = source_ml_source2_string_range_operand4;
                                            source_ml_source2_string_range_operand2.slice(source_ml_source2_string_range_position5, source_ml_source2_string_range_position6)
                                        }
                                    }
                                }
                            }
                        }).concat(&{
                            let source_ml_source2_character_argument16 = {
                                let source_ml_source2_string_unit_operand7 = source.clone();
                                {
                                    let source_ml_source2_string_unit_operand8 = end_2 - 1f64;
                                    (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text.length())) {
                                            f64::NAN
                                        } else {
                                            ({
                                                let source_ml_source2_string_unit_operand14 = {
                                                    let source_ml_source2_string_range_operand9 = text.clone();
                                                    {
                                                        let source_ml_source2_string_range_operand10 = index;
                                                        {
                                                            let source_ml_source2_string_range_operand11 = index + 1f64;
                                                            {
                                                                let source_ml_source2_string_range_position12 = source_ml_source2_string_range_operand10;
                                                                {
                                                                    let source_ml_source2_string_range_position13 = source_ml_source2_string_range_operand11;
                                                                    source_ml_source2_string_range_operand9.slice(source_ml_source2_string_range_position12, source_ml_source2_string_range_position13)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_string_unit_operand15 = 0f64;
                                                    source_ml_source2_string_unit_operand14.code_point_at(source_ml_source2_string_unit_operand15)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    } }))(source_ml_source2_string_unit_operand7, source_ml_source2_string_unit_operand8)
                                }
                            } - 1f64;
                            self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument16]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                        });
                        {
                            let index = end_2;
                            {
                                self::source_ml_source1_temporary_read_rust_integer_bound_predecessor_loop3(source, result, index)
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
pub fn read_rust_saturating_integer_bound_magnitude(
    bound: &self::ml_string::Text,
    bits: f64,
    signed: bool,
) -> self::ml_string::Text {
    {
        let limit = if signed {
            self::read_fixed_integer_cast_signed_limit(bits)
        } else {
            self::read_fixed_integer_cast_modulus(bits)
        };
        if limit == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else if (*bound)
            == (self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]))
        {
            if signed {
                limit.clone()
            } else {
                self::ml_string::Text::from_units(vec![48])
            }
        } else if (*bound)
            == (self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]))
        {
            self::read_rust_integer_bound_predecessor(&(limit))
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}
