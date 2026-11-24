// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342;
use super::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343;
use super::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Result;
use super::convert_java_script_big_integer_number;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::normalize_java_script_signed_integer;
use super::read_java_script_call_integer_arity;
use super::read_java_script_numeric_parsing_arity;
use super::read_java_script_source_exception_class;
use super::read_java_script_string_method_primitive_arity;
use super::source_ml_source1_temporary_parse_java_script_integer_prefix_loop1341;
use super::source_ml_source1_temporary_parse_java_script_integer_prefix_loop1344;
use super::test_java_script_call_unary_number_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_primitive_arity_error(
    operation: &self::ml_string::Text,
    count: f64,
) -> self::ml_string::Text {
    {
        let parsing_arity = self::read_java_script_numeric_parsing_arity(operation);
        if (parsing_arity >= (0f64)) {
            if (count == parsing_arity) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    110, 117, 109, 101, 114, 105, 99, 32, 112, 97, 114, 115, 105, 110, 103, 32,
                    114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 99, 111,
                    110, 118, 101, 114, 116, 101, 100, 32, 111, 112, 101, 114, 97, 110, 100,
                ])
            }
        } else {
            {
                let string_arity = self::read_java_script_string_method_primitive_arity(operation);
                if (string_arity >= (0f64)) {
                    if (count == string_arity) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![
                            83, 116, 114, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                            110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 118, 101, 114, 121,
                            32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116,
                            101, 100, 32, 111, 112, 101, 114, 97, 110, 100,
                        ])
                    }
                } else {
                    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64)))
                        || (count > (4096f64)))
                    {
                        self::ml_string::Text::from_units(vec![
                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 112, 101, 114,
                            97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101,
                            105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108,
                            108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103,
                            101, 116,
                        ])
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                116, 104, 114, 111, 119, 32, 114, 101, 115, 116, 114, 105, 99, 116,
                                101, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 99, 99,
                                101, 115, 115,
                            ])))
                        {
                            if (count == (0f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 70, 117,
                                    110, 99, 116, 105, 111, 110, 32, 97, 99, 99, 101, 115, 115, 32,
                                    105, 103, 110, 111, 114, 101, 115, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109,
                                    101, 110, 116, 115,
                                ])
                            }
                        } else {
                            if ((self::read_java_script_source_exception_class(operation))
                                != (self::ml_string::Text::from_units(vec![])))
                            {
                                if (count == (2f64)) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        97, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111,
                                        114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                        116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120,
                                        116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110,
                                        100, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103,
                                        101, 32, 112, 114, 101, 115, 101, 110, 99, 101,
                                    ])
                                }
                            } else {
                                {
                                    let integer_arity =
                                        self::read_java_script_call_integer_arity(operation);
                                    if (integer_arity != (0f64)) {
                                        if (count == integer_arity) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                                                105, 110, 115, 116, 114, 117, 99, 116, 105, 111,
                                                110, 115, 32, 114, 101, 113, 117, 105, 114, 101,
                                                32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117,
                                                97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32,
                                                99, 111, 111, 114, 100, 105, 110, 97, 116, 101,
                                            ])
                                        }
                                    } else {
                                        if ((((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                114, 101, 97, 100, 32, 109, 105, 110, 105, 109,
                                                117, 109, 32, 110, 117, 109, 98, 101, 114,
                                            ])))
                                            || ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    114, 101, 97, 100, 32, 109, 97, 120, 105, 109,
                                                    117, 109, 32, 110, 117, 109, 98, 101, 114,
                                                ]))))
                                            || ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 32,
                                                    116, 101, 120, 116, 32, 117, 110, 105, 116,
                                                    115,
                                                ]))))
                                        {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            if self::test_java_script_call_unary_number_operation(
                                                operation,
                                            ) {
                                                if (count == (1f64)) {
                                                    self::ml_string::Text::from_units(vec![])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![
                                                        112, 114, 105, 109, 105, 116, 105, 118,
                                                        101, 32, 105, 110, 115, 116, 114, 117, 99,
                                                        116, 105, 111, 110, 115, 32, 114, 101, 113,
                                                        117, 105, 114, 101, 32, 101, 118, 101, 114,
                                                        121, 32, 97, 99, 116, 117, 97, 108, 32,
                                                        111, 112, 101, 114, 97, 110, 100, 32, 99,
                                                        111, 111, 114, 100, 105, 110, 97, 116, 101,
                                                    ])
                                                }
                                            } else {
                                                {
                                                    let unary = std::rc::Rc::new(
                                                        std::cell::RefCell::new(vec![
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 65, 114, 114, 97, 121,
                                                                    32, 99, 114, 101, 97, 116, 105,
                                                                    111, 110, 32, 108, 101, 110,
                                                                    103, 116, 104,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 86, 56, 32, 97, 114,
                                                                    114, 97, 121, 32, 106, 111,
                                                                    105, 110, 32, 108, 101, 110,
                                                                    103, 116, 104,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 110, 115, 116, 114,
                                                                    117, 99, 116, 32, 83, 121, 109,
                                                                    98, 111, 108, 32, 100, 101,
                                                                    115, 99, 114, 105, 112, 116,
                                                                    105, 118, 101, 32, 116, 101,
                                                                    120, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 111, 117, 110, 100, 32,
                                                                    98, 105, 110, 97, 114, 121, 51,
                                                                    50, 32, 110, 117, 109, 98, 101,
                                                                    114,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 117, 110, 116, 32,
                                                                    108, 101, 97, 100, 105, 110,
                                                                    103, 32, 122, 101, 114, 111,
                                                                    32, 98, 105, 116, 115,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 97, 100, 32, 105,
                                                                    110, 116, 101, 103, 101, 114,
                                                                    32, 105, 110, 100, 101, 120,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 99, 97, 108, 108, 97,
                                                                    98, 108, 101, 32, 116, 97, 114,
                                                                    103, 101, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 97, 114, 114, 97, 121,
                                                                    32, 108, 105, 107, 101, 32, 97,
                                                                    114, 103, 117, 109, 101, 110,
                                                                    116, 115, 32, 79, 98, 106, 101,
                                                                    99, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 79, 98, 106, 101, 99,
                                                                    116, 32, 97, 115, 115, 105,
                                                                    103, 110, 32, 116, 97, 114,
                                                                    103, 101, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 79, 98, 106, 101, 99,
                                                                    116, 32, 114, 101, 102, 108,
                                                                    101, 99, 116, 105, 111, 110,
                                                                    32, 116, 97, 114, 103, 101,
                                                                    116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 110, 111, 110, 110,
                                                                    117, 108, 108, 32, 79, 98, 106,
                                                                    101, 99, 116, 32, 105, 110,
                                                                    112, 117, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 111, 98, 106, 101, 99,
                                                                    116, 32, 102, 111, 114, 32,
                                                                    105, 110,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 111, 114, 100, 105,
                                                                    110, 97, 114, 121, 32, 112,
                                                                    114, 111, 112, 101, 114, 116,
                                                                    121, 32, 111, 98, 106, 101, 99,
                                                                    116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 97, 114, 114, 97, 121,
                                                                    32, 109, 101, 116, 104, 111,
                                                                    100, 32, 111, 98, 106, 101, 99,
                                                                    116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 97, 100, 32, 97, 114,
                                                                    114, 97, 121, 32, 108, 105,
                                                                    107, 101, 32, 108, 101, 110,
                                                                    103, 116, 104,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 100, 101, 108, 101,
                                                                    116, 101, 32, 115, 117, 99, 99,
                                                                    101, 115, 115,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 99, 111, 108, 108,
                                                                    101, 99, 116, 105, 111, 110,
                                                                    32, 97, 100, 100, 101, 114,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 77, 97, 112, 32, 101,
                                                                    110, 116, 114, 121, 32, 111,
                                                                    98, 106, 101, 99, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 113, 117, 105, 114,
                                                                    101, 32, 111, 98, 106, 101, 99,
                                                                    116, 32, 99, 111, 101, 114, 99,
                                                                    105, 98, 108, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    116, 101, 115, 116, 32, 111,
                                                                    98, 106, 101, 99, 116, 32, 118,
                                                                    97, 108, 117, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 97, 100, 32, 115,
                                                                    101, 109, 97, 110, 116, 105,
                                                                    99, 32, 116, 121, 112, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    114, 101, 97, 100, 32, 116,
                                                                    101, 120, 116, 32, 108, 101,
                                                                    110, 103, 116, 104,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    116, 114, 105, 109, 32, 116,
                                                                    101, 120, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    116, 114, 105, 109, 32, 116,
                                                                    101, 120, 116, 32, 115, 116,
                                                                    97, 114, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    116, 114, 105, 109, 32, 116,
                                                                    101, 120, 116, 32, 101, 110,
                                                                    100,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 110, 118, 101, 114,
                                                                    116, 32, 101, 120, 112, 108,
                                                                    105, 99, 105, 116, 32, 110,
                                                                    117, 109, 98, 101, 114,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 101, 114, 99, 101, 32,
                                                                    112, 114, 105, 109, 105, 116,
                                                                    105, 118, 101, 32, 110, 117,
                                                                    109, 98, 101, 114,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 101, 114, 99, 101, 32,
                                                                    112, 114, 105, 109, 105, 116,
                                                                    105, 118, 101, 32, 110, 117,
                                                                    109, 101, 114, 105, 99,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 110, 118, 101, 114,
                                                                    116, 32, 112, 114, 105, 109,
                                                                    105, 116, 105, 118, 101, 32,
                                                                    118, 97, 108, 117, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 110, 118, 101, 114,
                                                                    116, 32, 110, 117, 109, 98,
                                                                    101, 114, 32, 112, 114, 105,
                                                                    109, 105, 116, 105, 118, 101,
                                                                    32, 118, 97, 108, 117, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    105, 110, 99, 114, 101, 109,
                                                                    101, 110, 116, 32, 110, 117,
                                                                    109, 101, 114, 105, 99, 32,
                                                                    118, 97, 108, 117, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    100, 101, 99, 114, 101, 109,
                                                                    101, 110, 116, 32, 110, 117,
                                                                    109, 101, 114, 105, 99, 32,
                                                                    118, 97, 108, 117, 101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    110, 101, 103, 97, 116, 101,
                                                                    32, 110, 117, 109, 101, 114,
                                                                    105, 99, 32, 118, 97, 108, 117,
                                                                    101,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 110, 118, 101, 114,
                                                                    116, 32, 112, 114, 105, 109,
                                                                    105, 116, 105, 118, 101, 32,
                                                                    116, 101, 120, 116,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 110, 118, 101, 114,
                                                                    116, 32, 101, 120, 112, 108,
                                                                    105, 99, 105, 116, 32, 105,
                                                                    110, 116, 101, 103, 101, 114,
                                                                ],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 101, 114, 99, 101, 32,
                                                                    112, 114, 105, 109, 105, 116,
                                                                    105, 118, 101, 32, 105, 110,
                                                                    116, 101, 103, 101, 114,
                                                                ],
                                                            ),
                                                        ]),
                                                    );
                                                    {
                                                        let binary = std::rc::Rc::new(
                                                            std::cell::RefCell::new(vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 97,
                                                                        114, 114, 97, 121, 32, 100,
                                                                        101, 108, 101, 116, 105,
                                                                        111, 110, 32, 99, 111, 117,
                                                                        110, 116,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 99,
                                                                        104, 101, 99, 107, 101,
                                                                        100, 32, 97, 114, 114, 97,
                                                                        121, 32, 114, 101, 112,
                                                                        108, 97, 99, 101, 109, 101,
                                                                        110, 116, 32, 105, 110,
                                                                        100, 101, 120,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        109, 117, 108, 116, 105,
                                                                        112, 108, 121, 32, 115,
                                                                        105, 103, 110, 101, 100,
                                                                        32, 105, 110, 116, 101,
                                                                        103, 101, 114, 32, 98, 105,
                                                                        116, 115,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 114,
                                                                        101, 108, 97, 116, 105,
                                                                        118, 101, 32, 97, 114, 114,
                                                                        97, 121, 32, 105, 110, 100,
                                                                        101, 120,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 99,
                                                                        104, 101, 99, 107, 101,
                                                                        100, 32, 97, 114, 114, 97,
                                                                        121, 32, 97, 112, 112, 101,
                                                                        110, 100, 32, 108, 101,
                                                                        110, 103, 116, 104,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        99, 111, 109, 112, 97, 114,
                                                                        101, 32, 114, 101, 108, 97,
                                                                        116, 105, 111, 110, 97,
                                                                        108, 32, 118, 97, 108, 117,
                                                                        101, 115,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        115, 117, 98, 116, 114, 97,
                                                                        99, 116, 32, 110, 117, 109,
                                                                        101, 114, 105, 99, 32, 118,
                                                                        97, 108, 117, 101, 115,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        109, 117, 108, 116, 105,
                                                                        112, 108, 121, 32, 110,
                                                                        117, 109, 101, 114, 105,
                                                                        99, 32, 118, 97, 108, 117,
                                                                        101, 115,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        100, 105, 118, 105, 100,
                                                                        101, 32, 110, 117, 109,
                                                                        101, 114, 105, 99, 32, 118,
                                                                        97, 108, 117, 101, 115,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 110,
                                                                        117, 109, 101, 114, 105,
                                                                        99, 32, 114, 101, 109, 97,
                                                                        105, 110, 100, 101, 114,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 97, 105, 115, 101, 32,
                                                                        110, 117, 109, 101, 114,
                                                                        105, 99, 32, 112, 111, 119,
                                                                        101, 114,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 112, 101, 97,
                                                                        116, 32, 116, 101, 120,
                                                                        116,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 116,
                                                                        101, 120, 116, 32, 99, 111,
                                                                        100, 101, 32, 112, 111,
                                                                        105, 110, 116,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 116,
                                                                        101, 120, 116, 32, 99, 104,
                                                                        97, 114, 97, 99, 116, 101,
                                                                        114,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        114, 101, 97, 100, 32, 114,
                                                                        101, 108, 97, 116, 105,
                                                                        118, 101, 32, 116, 101,
                                                                        120, 116, 32, 99, 104, 97,
                                                                        114, 97, 99, 116, 101, 114,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        99, 111, 110, 99, 97, 116,
                                                                        101, 110, 97, 116, 101, 32,
                                                                        116, 101, 120, 116,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        99, 111, 109, 112, 97, 114,
                                                                        101, 32, 116, 101, 120,
                                                                        116,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        116, 101, 115, 116, 32,
                                                                        116, 101, 120, 116, 32,
                                                                        112, 114, 101, 102, 105,
                                                                        120,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        116, 101, 115, 116, 32,
                                                                        116, 101, 120, 116, 32,
                                                                        115, 117, 102, 102, 105,
                                                                        120,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        116, 101, 115, 116, 32,
                                                                        116, 101, 120, 116, 32,
                                                                        105, 110, 99, 108, 117,
                                                                        115, 105, 111, 110,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        97, 100, 100, 32, 112, 114,
                                                                        105, 109, 105, 116, 105,
                                                                        118, 101, 32, 118, 97, 108,
                                                                        117, 101, 115,
                                                                    ],
                                                                ),
                                                            ]),
                                                        );
                                                        {
                                                            let ternary = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100]), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116])]));
                                                            {
                                                                let arity =
                                                                    if self::ml_array::includes(
                                                                        &{
                                                                            let ml_reference_value2013 =
                                                                                unary
                                                                                    .borrow()
                                                                                    .clone();
                                                                            ml_reference_value2013
                                                                        },
                                                                        operation,
                                                                        0f64,
                                                                        false,
                                                                        |a, b| a == b,
                                                                    ) {
                                                                        1f64
                                                                    } else {
                                                                        if self::ml_array::includes(
                                                                            &{
                                                                                let ml_reference_value2014 =
                                                                                    binary
                                                                                        .borrow()
                                                                                        .clone();
                                                                                ml_reference_value2014
                                                                            },
                                                                            operation,
                                                                            0f64,
                                                                            false,
                                                                            |a, b| a == b,
                                                                        ) {
                                                                            2f64
                                                                        } else {
                                                                            if self::ml_array::includes(&{ let ml_reference_value2015 = ternary.borrow().clone(); ml_reference_value2015 }, operation, 0f64, false, |a, b| a == b) {
                                                                            3f64
                                                                        } else {
                                                                            (-1f64)
                                                                        }
                                                                        }
                                                                    };
                                                                if (arity < (0f64)) {
                                                                    self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                                                                } else {
                                                                    if (count != arity) {
                                                                        self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![])
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
pub fn read_java_script_radix_digit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (90f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (122f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn parse_java_script_integer_prefix(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    radix_number: f64,
) -> f64 {
    {
        let start = 0f64;
        {
            let start_2 =
                self::source_ml_source1_temporary_parse_java_script_integer_prefix_loop1341(
                    units.clone(),
                    start,
                );
            {
                let negative = false;
                {
                    let sign = ({
                        let ml_reference_array2016 = units.clone();
                        let ml_reference_position2016 = start_2;
                        let ml_reference_projection2016 = self::ml_array::optional_at(
                            &*ml_reference_array2016.borrow(),
                            self::ml_array::number_index(ml_reference_position2016),
                        );
                        ml_reference_projection2016
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                    {
                        let source_ml_source2_s32512 = if ((sign == (43f64)) || (sign == (45f64))) {
                            {
                                let negative_2 = (sign == (45f64));
                                {
                                    let start_3 = (start_2 + 1f64);
                                    self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342Next(start_3, negative_2)
                                }
                            }
                        } else {
                            self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342Next(start_2, negative)
                        };
                        match source_ml_source2_s32512.clone() {
                            self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342Next(start_4, negative_3) => {
                                {
                                    let radix = self::normalize_java_script_signed_integer(radix_number);
                                    {
                                        let strip = ((radix == (0f64)) || (radix == (16f64)));
                                        if ((radix != (0f64)) && ((radix < (2f64)) || (radix > (36f64)))) {
                                            f64::NAN
                                        } else {
                                            {
                                                let radix_3 = if (radix == (0f64)) {
                                                    {
                                                        let radix_2 = 10f64;
                                                        radix_2
                                                    }
                                                } else {
                                                    radix
                                                };
                                                {
                                                    let source_ml_source2_s32525 = if ((strip && ((({ let ml_reference_array2017 = units.clone(); let ml_reference_position2017 = start_4; let ml_reference_projection2017 = self::ml_array::optional_at(&*ml_reference_array2017.borrow(), self::ml_array::number_index(ml_reference_position2017)); ml_reference_projection2017 }).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((({
                                                        let source_ml_source2_array_read_index32533 = (start_4 + 1f64);
                                                        { let ml_reference_array2018 = units.clone(); let ml_reference_position2018 = source_ml_source2_array_read_index32533; let ml_reference_projection2018 = self::ml_array::optional_at(&*ml_reference_array2018.borrow(), self::ml_array::number_index(ml_reference_position2018)); ml_reference_projection2018 }
                                                    }).clone().unwrap_or_else(|| (-1f64))) == (120f64)) || ((({
                                                        let source_ml_source2_array_read_index32537 = (start_4 + 1f64);
                                                        { let ml_reference_array2019 = units.clone(); let ml_reference_position2019 = source_ml_source2_array_read_index32537; let ml_reference_projection2019 = self::ml_array::optional_at(&*ml_reference_array2019.borrow(), self::ml_array::number_index(ml_reference_position2019)); ml_reference_projection2019 }
                                                    }).clone().unwrap_or_else(|| (-1f64))) == (88f64)))) {
                                                        {
                                                            let radix_4 = 16f64;
                                                            {
                                                                let start_5 = (start_4 + 2f64);
                                                                self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343Next(start_5, radix_4)
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343Next(start_4, radix_3)
                                                    };
                                                    match source_ml_source2_s32525.clone() {
                                                        self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343Next(start_6, radix_5) => {
                                                            {
                                                                let value = self::ml::Big::from_i128(0);
                                                                {
                                                                    let count = 0f64;
                                                                    {
                                                                        let base = {
                                                                            let source_ml_source2_integer_result32539 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(radix_5);
                                                                            if (((source_ml_source2_integer_result32539.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result32539, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(radix_5)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_s32540 = self::source_ml_source1_temporary_parse_java_script_integer_prefix_loop1344(units.clone(), radix_5, value.clone(), count, base.clone(), start_6);
                                                                            match source_ml_source2_s32540.clone() {
                                                                                self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Result::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Done(value_2, count_2) => {
                                                                                    if (count_2 == (0f64)) {
                                                                                        f64::NAN
                                                                                    } else {
                                                                                        {
                                                                                            let result = self::convert_java_script_big_integer_number(value_2.clone());
                                                                                            if negative_3 {
                                                                                                (-result)
                                                                                            } else {
                                                                                                result
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
    }
}
