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
    (
        "grammar-range-candidate",
        &[
            ("name", FieldType::Name),
            ("mapping", FieldType::Name),
            ("ranges", FieldType::Expressions),
        ],
    ),
    (
        "source-range",
        &[("start", FieldType::Integer), ("end", FieldType::Integer)],
    ),
    (
        "grammar-field-candidate",
        &[
            ("name", FieldType::Name),
            ("mapping", FieldType::Name),
            ("selector", FieldType::Expression),
        ],
    ),
    (
        "source-field-selector",
        &[
            ("label", FieldType::Name),
            ("mode", FieldType::Choice(&["include", "exclude"])),
            ("first", FieldType::Integer),
            ("count", FieldType::Integer),
        ],
    ),
];
