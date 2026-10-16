// Generated from the shared grammar selection schema. Do not edit.
use super::feature::{FieldType, FormFields};

pub const SELECTION_EXPRESSION_FORMS: &[(&str, FormFields)] = &[
    (
        "any-grammar",
        &[
            ("item", FieldType::Expression),
            ("items", FieldType::Expressions),
        ],
    ),
    (
        "all-grammar",
        &[
            ("item", FieldType::Expression),
            ("items", FieldType::Expressions),
        ],
    ),
    (
        "grammar-candidate",
        &[("name", FieldType::Name), ("mapping", FieldType::Name)],
    ),
];
