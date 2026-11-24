//! Bounded original owner-tree transport through shared registered edges only.
use crate::translation::rust_pointer_integer_owner_rules::accept_rust_pointer_integer_owner_child;
use crate::translation::source_signatures::SourceOwner;
pub(in super::super) fn element_owner(mut owner: Option<&SourceOwner>) -> Option<&SourceOwner> {
    for _ in 0..128 {
        let value = owner?;
        if !accept_rust_pointer_integer_owner_child(
            &value.kind,
            &value.path.join("::"),
            super::count(value.arguments.len()),
        ) {
            return owner;
        }
        owner = value.arguments.first();
    }
    None
}

pub(super) fn clone_facts(
    element: Option<&super::Type>,
    owner: Option<&SourceOwner>,
    cloned: bool,
) -> (String, bool) {
    if !cloned {
        return (String::new(), false);
    }
    let (bits, signed) = match element {
        Some(super::Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    let error =
        crate::translation::rust_array_lookup_rules::read_rust_array_lookup_cloned_element_error(
            element.map_or("", super::Type::kind),
            &bits,
            signed,
            owner.map_or("", |owner| owner.kind.as_str()),
            &owner.map_or_else(String::new, |owner| owner.path.join("::")),
            owner.map_or(-1.0, |owner| super::count(owner.arguments.len())),
        );
    let string = element.is_some_and(|element| element.kind() == "string") && error.is_empty();
    (error, string)
}
pub(in super::super) fn scalar_owner(source: &super::SourceOrigin) -> String {
    if let Some(expanded) = &source.expanded_source_owner {
        super::SourceOrigin {
            source_owner: Some(expanded.clone()),
            ..super::SourceOrigin::unknown()
        }
        .pointer_integer_owner()
    } else {
        source.pointer_integer_owner()
    }
}
include!("../../../../../tests/internal/translation/rust/expressions/array_lookups/source_owners.rs");
