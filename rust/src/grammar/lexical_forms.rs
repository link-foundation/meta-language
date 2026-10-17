// Generated from the shared grammar-lexer-rules.lino policy. Do not edit.
use super::feature::{FieldType, FormFields};

pub const LEXICAL_EXPRESSION_FORMS: &[(&str, FormFields)] = &[
    (
        "lexicalScope",
        &[
            ("name", FieldType::Name),
            ("owner", FieldType::Name),
            ("item", FieldType::Expression),
        ],
    ),
    (
        "lexicalToken",
        &[("name", FieldType::Name), ("token", FieldType::Name)],
    ),
    (
        "lexicalOwner",
        &[("name", FieldType::Name), ("item", FieldType::Expression)],
    ),
    (
        "lexicalBinding",
        &[("name", FieldType::Name), ("item", FieldType::Expression)],
    ),
    (
        "lexicalLiteral",
        &[("value", FieldType::Text), ("name", FieldType::Name)],
    ),
    (
        "lexicalRule",
        &[
            ("name", FieldType::Name),
            ("type", FieldType::Name),
            ("mode", FieldType::Name),
            ("channel", FieldType::Name),
            ("disposition", FieldType::Name),
            ("item", FieldType::Expression),
            ("action", FieldType::Block("action", false)),
        ],
    ),
    (
        "lexicalFragment",
        &[("name", FieldType::Name), ("item", FieldType::Expression)],
    ),
    (
        "lexicalObligation",
        &[
            ("name", FieldType::Name),
            ("category", FieldType::Name),
            ("source", FieldType::Text),
        ],
    ),
    ("lexicalChannel", &[("name", FieldType::Name)]),
    ("lexicalMode", &[("name", FieldType::Name)]),
    ("lexicalType", &[("name", FieldType::Name)]),
];
