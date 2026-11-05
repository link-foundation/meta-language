// Generated from complete js/src/translation/rust-primitive-associated-call-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::ml_string;
use super::test_rust_checked_integer_lazy_fallback;
use super::test_rust_checked_integer_mapping_consumer;
use super::test_rust_checked_integer_predicate_consumer;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_from_source(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (((((((((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
        && ((*bits) == (self::ml_string::Text::from_units(vec![]))))
        && !signed)
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101,
            ]))))
        && ((*path) == (self::ml_string::Text::from_units(vec![102, 54, 52]))))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        || self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_from(
    target: &self::ml_string::Text,
    source: &self::ml_string::Text,
) -> self::ml_string::Text {
    if !(self::accept_rust_primitive_from_method(
        target,
        &(self::ml_string::Text::from_units(vec![102, 114, 111, 109])),
    )) || !(self::accept_rust_primitive_from_method(
        source,
        &(self::ml_string::Text::from_units(vec![102, 114, 111, 109])),
    )) {
        self::ml_string::Text::from_units(vec![])
    } else if (*target) == (*source) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if (((*source) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
        && ((*target) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        && ((*target) != (self::ml_string::Text::from_units(vec![102, 54, 52])))
    {
        self::ml_string::Text::from_units(vec![
            98, 111, 111, 108, 101, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114,
        ])
    } else if ((*target) == (self::ml_string::Text::from_units(vec![102, 54, 52])))
        && self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![105, 56]),
                self::ml_string::Text::from_units(vec![105, 49, 54]),
                self::ml_string::Text::from_units(vec![105, 51, 50]),
                self::ml_string::Text::from_units(vec![117, 56]),
                self::ml_string::Text::from_units(vec![117, 49, 54]),
                self::ml_string::Text::from_units(vec![117, 51, 50]),
            ],
            source,
            0f64,
            false,
            |a, b| a == b,
        )
    {
        self::ml_string::Text::from_units(vec![
            102, 108, 111, 97, 116, 105, 110, 103, 32, 101, 120, 97, 99, 116,
        ])
    } else {
        {
            let admitted = (((((((((*source)
                == (self::ml_string::Text::from_units(vec![117, 56])))
                && self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![117, 49, 54]),
                        self::ml_string::Text::from_units(vec![117, 51, 50]),
                        self::ml_string::Text::from_units(vec![117, 54, 52]),
                        self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                        self::ml_string::Text::from_units(vec![105, 49, 54]),
                        self::ml_string::Text::from_units(vec![105, 51, 50]),
                        self::ml_string::Text::from_units(vec![105, 54, 52]),
                        self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                    ],
                    target,
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                || (((*source) == (self::ml_string::Text::from_units(vec![117, 49, 54])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![117, 51, 50]),
                            self::ml_string::Text::from_units(vec![117, 54, 52]),
                            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                            self::ml_string::Text::from_units(vec![105, 51, 50]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![117, 51, 50])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![117, 54, 52]),
                            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![117, 54, 52])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 56])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 49, 54]),
                            self::ml_string::Text::from_units(vec![105, 51, 50]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 49, 54])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 51, 50]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 51, 50])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 54, 52])))
                    && ((*target) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56]))));
            if admitted {
                self::ml_string::Text::from_units(vec![
                    105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 97, 99, 116,
                ])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_predicate(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> bool {
    (((((*kind)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (parameters == 1f64))
        && (layers == 0f64))
        && parameter)
        && result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_fallback(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    result: bool,
) -> bool {
    ((((*kind)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (parameters == 0f64))
        && (layers == 0f64))
        && result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_value_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 97, 105, 115, 101, 82, 117, 115, 116, 80, 97, 110, 105, 99, 86, 97, 108, 117, 101,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_rust_panic_value(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> bool {
    ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 116, 104])))
        && ((*operation) == (self::read_rust_panic_value_operation()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_value_argument_error(
    language: &self::ml_string::Text,
    count: f64,
    spread: bool,
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((((*language) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])))
        && (count == 1f64))
        && !spread)
        && ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 82, 117, 115, 116, 32, 112, 97, 110, 105, 99, 32, 118, 97, 108, 117, 101, 32,
            110, 101, 101, 100, 115, 32, 111, 110, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32,
            115, 111, 117, 114, 99, 101, 32, 83, 116, 114, 105, 110, 103, 32, 109, 101, 115, 115,
            97, 103, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_panic_value_expression(
    target: &self::ml_string::Text,
    message: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((self::ml_string::Text::from_units(vec![
            40, 40, 109, 108, 95, 109, 101, 115, 115, 97, 103, 101, 41, 61, 62, 123, 116, 104, 114,
            111, 119, 32, 110, 101, 119, 32, 69, 114, 114, 111, 114, 40, 109, 108, 95, 109, 101,
            115, 115, 97, 103, 101, 41, 59, 125, 41, 40,
        ]))
        .concat(message))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        ((self::ml_string::Text::from_units(vec![
            112, 97, 110, 105, 99, 33, 40, 34, 123, 125, 34, 44, 32,
        ]))
        .concat(message))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_panic_formatting_macro(name: &self::ml_string::Text, ordinary: bool) -> bool {
    ordinary
        && (((((*name) == (self::ml_string::Text::from_units(vec![112, 97, 110, 105, 99])))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100,
                ]))))
            || ((*name) == (self::ml_string::Text::from_units(vec![116, 111, 100, 111]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_panic_formatting_value(
    name: &self::ml_string::Text,
    ordinary: bool,
    kind: &self::ml_string::Text,
    count: f64,
    braces: bool,
) -> bool {
    (self::accept_rust_panic_formatting_macro(name, ordinary) && (count > 0f64))
        && ((count > 1f64)
            || (((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114]))) && braces))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_formatting_prefix(name: &self::ml_string::Text) -> self::ml_string::Text {
    if (*name)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110,
            116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99,
            111, 100, 101, 58, 32,
        ])
    } else if (*name)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            110, 111, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100, 58, 32,
        ])
    } else if (*name) == (self::ml_string::Text::from_units(vec![116, 111, 100, 111])) {
        self::ml_string::Text::from_units(vec![
            110, 111, 116, 32, 121, 101, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101,
            100, 58, 32,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_completion_kind(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::accept_linked_rust_panic_value(kind, operation) {
        self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116])
    } else {
        kind.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_message_source_error(
    consumer: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    storage: &self::ml_string::Text,
    layers: f64,
    known: bool,
    references: bool,
) -> self::ml_string::Text {
    if ((*consumer) != (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116])))
        || ((((((*kind)
            == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
            && (((*storage)
                == (self::ml_string::Text::from_units(vec![
                    115, 116, 114, 105, 110, 103, 32, 115, 108, 105, 99, 101,
                ])))
                || ((*storage)
                    == (self::ml_string::Text::from_units(vec![
                        111, 119, 110, 101, 100, 32, 115, 116, 114, 105, 110, 103,
                    ])))))
            && (layers >= 1f64))
            && known)
            && references)
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            101, 120, 112, 101, 99, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97,
            99, 116, 117, 97, 108, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 115, 116, 97,
            110, 100, 97, 114, 100, 32, 115, 116, 114, 32, 109, 101, 115, 115, 97, 103, 101, 32,
            115, 111, 117, 114, 99, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_option_panic_consumer(consumer: &self::ml_string::Text) -> bool {
    ((*consumer) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112])))
        || ((*consumer) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_option_panic_message(
    consumer: &self::ml_string::Text,
    message: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*consumer) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112])) {
        self::ml_string::Text::from_units(vec![
            99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119,
            114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118,
            97, 108, 117, 101,
        ])
    } else if (*consumer) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116]))
    {
        message.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_arity(method: &self::ml_string::Text, ordinary: bool) -> f64 {
    if ordinary {
        if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 117, 98,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 109, 117, 108,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 95, 101, 117, 99, 108, 105,
                    100,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109, 95, 101, 117, 99, 108, 105,
                    100,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
                ]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        ) {
            1f64
        } else if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 110, 101, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
                ]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        ) {
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
pub fn accept_rust_checked_integer_receiver(
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
    ((((self::read_rust_checked_integer_arity(method, true)) >= 0f64)
        && ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
        && (((*method)
            != (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
            ])))
            || signed))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_consumer_arity(consumer: &self::ml_string::Text) -> f64 {
    if ((((*consumer)
        == (self::ml_string::Text::from_units(vec![105, 115, 95, 115, 111, 109, 101])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![105, 115, 95, 110, 111, 110, 101]))))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ]))))
        || ((*consumer) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112])))
    {
        0f64
    } else if ((((*consumer)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*consumer) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116]))))
        || self::test_rust_checked_integer_lazy_fallback(consumer))
        || self::test_rust_checked_integer_predicate_consumer(consumer)
    {
        1f64
    } else if self::test_rust_checked_integer_mapping_consumer(consumer) {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_owner(
    method: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
            ])))
    {
        self::ml_string::Text::from_units(vec![117, 51, 50])
    } else {
        receiver.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_reason(
    method: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 105, 102, 116, 32, 99, 111,
            117, 110, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116,
            97, 110, 100, 97, 114, 100, 32, 117, 51, 50, 32, 111, 119, 110, 101, 114, 115, 104,
            105, 112,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 103, 117, 109, 101, 110,
            116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32,
            114, 101, 99, 101, 105, 118, 101, 114, 32, 116, 121, 112, 101, 32, 97, 110, 100, 32,
            115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_shift_operation(
    term: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*term)
        == (self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 32, 108, 101, 102, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101,
            66, 105, 116, 115, 76, 101, 102, 116,
        ])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 32, 114, 105, 103, 104, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101,
            66, 105, 116, 115, 82, 105, 103, 104, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
